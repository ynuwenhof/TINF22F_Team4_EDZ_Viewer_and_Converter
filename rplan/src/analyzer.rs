use super::context::Context;
use super::edz::{Address, Item, Manifest, PartContainer, PointContainer};
use super::error::Error;
use super::model::Sample;
use super::queue::{Entry, Status};
use super::{error, model};
use bson::oid::ObjectId;
use handlebars::Handlebars;
use libarchive::Ownership;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, ErrorKind, Read, Seek, Write};
use std::path::{Component, Path, PathBuf};
use time::ext::NumericalDuration;
use time::OffsetDateTime;
use tokio::io::AsyncReadExt;
use tokio::{fs, task};
use tracing::{debug, error_span, field, Instrument, Span};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

pub struct Analyzer;

impl Analyzer {
    pub fn new(ctx: Context, i: u8) -> Self {
        task::spawn(async move {
            loop {
                run(&ctx, i).await;
            }
        });

        Self
    }
}

async fn run(ctx: &Context, task_id: u8) {
    let span = error_span!("analyzer", task = task_id, id = field::Empty);

    let entry = match ctx.queue.push_analyzer().await {
        Ok(entry) => entry,
        Err(_) => return,
    };

    async {
        if let Err(e) = process(ctx, entry).await {
            error!("{e}");
        }
    }
    .instrument(span)
    .await;
}

async fn process(ctx: &Context, entry: Entry) -> error::Result<()> {
    let span = Span::current();
    span.record("id", field::display(&entry.task.hash));
    debug!("started processing job");

    let task_hash = entry.task.hash.clone();
    let output_path = ctx.cli.data_path.join(&task_hash);
    if let Err(e) = analyze(ctx, entry, &output_path).await {
        error!("{e}");

        ctx.queue.set_status(task_hash, Status::Failed).await;

        let res = fs::remove_dir_all(&output_path).await;
        if let Err(e) = res {
            if e.kind() != ErrorKind::NotFound {
                return Err(e.into());
            }
        }
    }

    debug!("finished processing job");

    Ok(())
}

const JFIF_SOI: [u8; 2] = [0xFF, 0xD8];

async fn analyze<P: AsRef<Path>>(ctx: &Context, entry: Entry, path: P) -> error::Result<()> {
    let sample_path = path.as_ref();

    debug!("started decompressing sample");

    let mut temp_file = entry.file;
    task::block_in_place(|| decompress(&mut temp_file, sample_path))?;

    debug!("finished decompressing sample");
    debug!("started analyzing sample");

    let manifest = Manifest::from_file(sample_path.join("manifest.xml")).await?;

    let sample_id = ObjectId::new();
    let packages = manifest.packages.len();

    let xml_path = sample_path.join("items/partxml").canonicalize()?;
    let pic_path = sample_path.join("items/picture").canonicalize()?;

    let mut template = Template {
        name: entry.task.name.clone(),
        ..Default::default()
    };

    let mut cache = HashMap::new();
    let mut session = ctx.db.start_session().await?;

    for (i, package) in manifest.packages.into_iter().enumerate() {
        let items = package.items;
        let mut package = model::Package::new(sample_id, i, package.kind, package.name);

        let mut template_package = Package {
            name: package.name.clone(),
            ..Default::default()
        };

        for item in items {
            match item {
                Item::Manufacturer(data) => {
                    let relative_mfr_path = normalize_path(&data.path)?;
                    let absolute_mfr_path = xml_path.join(&relative_mfr_path);

                    if let Some(id) = cache.get(&absolute_mfr_path) {
                        package.manufacturer_id = Some(*id);
                        continue;
                    }

                    let addr = Address::from_file(&absolute_mfr_path).await?;
                    let company = model::Company::new(addr.attributes());
                    package.manufacturer_id = Some(company.id);

                    template_package.manufacturer = Some(company.map.clone());

                    ctx.db
                        .insert_company_with_session(&company, &mut session)
                        .await?;
                    cache.insert(absolute_mfr_path, company.id);
                }
                Item::Supplier(data) => {
                    let relative_supplier_path = normalize_path(&data.path)?;
                    let absolute_supplier_path = xml_path.join(&relative_supplier_path);

                    if let Some(id) = cache.get(&absolute_supplier_path) {
                        package.supplier_id = Some(*id);
                        continue;
                    }

                    let addr = Address::from_file(&absolute_supplier_path).await?;
                    let company = model::Company::new(addr.attributes());
                    package.supplier_id = Some(company.id);

                    template_package.supplier = Some(company.map.clone());

                    ctx.db
                        .insert_company_with_session(&company, &mut session)
                        .await?;
                    cache.insert(absolute_supplier_path, company.id);
                }
                Item::Part(data) => {
                    let relative_part_path = normalize_path(&data.path)?;
                    let absolute_part_path = xml_path.join(&relative_part_path);

                    let part = PartContainer::from_file(&absolute_part_path).await?;
                    package.part_id = Some(part.id);

                    let var = Variant {
                        attrs: part.part.variant.attributes.clone(),
                        function_templates: part
                            .part
                            .variant
                            .function_templates
                            .iter()
                            .map(|f| f.0.clone())
                            .collect(),
                    };

                    let mut a = part.attributes.clone();
                    a.extend(part.part.attributes.clone());
                    template_package.part = Some(Part {
                        attrs: a,
                        free_properties: part
                            .part
                            .free_properties
                            .iter()
                            .map(|f| f.0.clone())
                            .collect(),
                        variant: var,
                    });

                    ctx.db.insert_part_with_session(&part, &mut session).await?;
                }
                Item::ConnectionPoints(data) => {
                    let relative_point_path = normalize_path(&data.path)?;
                    let absolute_point_path = xml_path.join(&relative_point_path);

                    let point = PointContainer::from_file(&absolute_point_path).await?;
                    package.point_id = Some(point.id);

                    let mut a = point.attributes.clone();
                    a.extend(point.terminal.attributes.clone());
                    template_package.point = Some(Point {
                        attrs: a,
                        terminal_positions: point
                            .terminal
                            .terminal_positions
                            .iter()
                            .map(|t| t.0.clone())
                            .collect(),
                    });

                    ctx.db
                        .insert_point_with_session(&point, &mut session)
                        .await?;
                }
                Item::PictureFile(data) => {
                    let relative_img_path = normalize_path(&data.path)?;
                    let absolute_img_path = pic_path.join(&relative_img_path);

                    let mut file = fs::File::options()
                        .read(true)
                        .open(&absolute_img_path)
                        .await?;

                    let mut buf = [0u8; 2];
                    file.read_exact(&mut buf).await?;

                    if buf != JFIF_SOI {
                        return Err(Error::InvalidJfifSignature);
                    }

                    package.image = Some(relative_img_path);
                }
                Item::Unknown => {}
            }
        }

        template.packages.push(template_package);

        ctx.db
            .insert_package_with_session(&package, &mut session)
            .await?;
    }

    if let Some(parent) = sample_path.parent() {
        let aasx_path = parent.join(format!("{}.aasx", entry.task.hash));
        task::block_in_place(|| convert(&template, &aasx_path))?;
    }

    let completed = OffsetDateTime::now_utc();
    let expires = completed
        .checked_add(1.days())
        .ok_or(Error::DateOutOfRange)?;

    let sample = Sample {
        id: sample_id,
        hash: entry.task.hash,
        name: entry.task.name,
        size: entry.task.size,
        packages,
        created: entry.task.created,
        completed,
        expires,
    };

    ctx.db
        .insert_sample_with_session(&sample, &mut session)
        .await?;

    session.commit_transaction().await?;
    ctx.queue.remove_task(sample.hash).await;

    debug!("finished analyzing sample");

    Ok(())
}

const SEVENZ_SIGNATURE: [u8; 6] = [b'7', b'z', 0xBC, 0xAF, 0x27, 0x1C];

fn decompress<P: AsRef<Path>>(file: &mut File, path: P) -> error::Result<()> {
    file.rewind()?;
    let mut buf = [0u8; 6];
    file.read_exact(&mut buf)?;

    if buf != SEVENZ_SIGNATURE {
        return Err(Error::Invalid7zSignature);
    }

    file.rewind()?;
    libarchive::uncompress_archive(file, path.as_ref(), Ownership::Ignore)?;

    Ok(())
}

fn normalize_path(s: &str) -> error::Result<PathBuf> {
    let path = PathBuf::from(s.replace('\\', "/"));

    let invalid_component = path
        .components()
        .any(|c| !matches!(c, Component::Normal(_)));

    if invalid_component {
        return Err(Error::PathComponentDenied);
    }

    Ok(path)
}

const TEMPLATE_HBS: &str = include_str!("../../assets/template.hbs");
const TEMPLATE_AASX: &[u8] = include_bytes!("../../assets/template.aasx");

#[derive(Default, Serialize)]
struct Template {
    name: String,
    packages: Vec<Package>,
}

#[derive(Default, Serialize)]
struct Package {
    name: String,
    manufacturer: Option<HashMap<String, String>>,
    supplier: Option<HashMap<String, String>>,
    part: Option<Part>,
    point: Option<Point>,
}

#[derive(Default, Serialize)]
struct Part {
    attrs: HashMap<String, String>,
    free_properties: Vec<HashMap<String, String>>,
    variant: Variant,
}

#[derive(Default, Serialize)]
struct Variant {
    attrs: HashMap<String, String>,
    function_templates: Vec<HashMap<String, String>>,
}

#[derive(Default, Serialize)]
struct Point {
    attrs: HashMap<String, String>,
    terminal_positions: Vec<HashMap<String, String>>,
}

fn convert<P: AsRef<Path>>(template: &Template, path: P) -> error::Result<()> {
    use std::fs;

    let handlebars = Handlebars::new();
    let s = handlebars.render_template(TEMPLATE_HBS, template)?;

    let buf = TEMPLATE_AASX.to_vec();
    let cur = Cursor::new(buf);

    let mut zip = ZipWriter::new_append(cur)?;
    let options = SimpleFileOptions::default();

    let name =
        "aasx/AssetAdministrationShell---10A74E49/AssetAdministrationShell---10A74E49.aas.xml";
    zip.start_file(name, options)?;
    zip.write_all(s.as_bytes())?;
    let buf = zip.finish()?;

    fs::write(path, buf.into_inner())?;
    Ok(())
}
