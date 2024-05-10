use super::edz::{PartContainer, PointContainer};
use super::error;
use super::model::{Company, Package, Sample};
use bson::oid::ObjectId;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use mongodb::{Client, ClientSession, Collection, IndexModel};

#[derive(Clone)]
pub struct Database {
    client: Client,
    samples: Collection<Sample>,
    packages: Collection<Package>,
    companies: Collection<Company>,
    parts: Collection<PartContainer>,
    points: Collection<PointContainer>,
}

impl Database {
    pub async fn new(client: Client) -> error::Result<Self> {
        let db = client.database("rplan");
        let samples = db.collection("samples");
        let packages = db.collection("packages");
        let companies = db.collection("companies");
        let parts = db.collection("parts");
        let points = db.collection("points");

        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder()
            .keys(doc!("hash": -1))
            .options(options)
            .build();

        samples.create_index(index, None).await?;

        Ok(Self {
            client,
            samples,
            packages,
            companies,
            parts,
            points,
        })
    }

    pub async fn start_session(&self) -> error::Result<ClientSession> {
        let mut session = self.client.start_session(None).await?;
        session.start_transaction(None).await?;
        Ok(session)
    }

    pub async fn insert_sample_with_session(
        &self,
        sample: &Sample,
        session: &mut ClientSession,
    ) -> error::Result<()> {
        self.samples
            .insert_one_with_session(sample, None, session)
            .await?;
        Ok(())
    }

    pub async fn insert_package_with_session(
        &self,
        package: &Package,
        session: &mut ClientSession,
    ) -> error::Result<()> {
        self.packages
            .insert_one_with_session(package, None, session)
            .await?;
        Ok(())
    }

    pub async fn insert_company_with_session(
        &self,
        company: &Company,
        session: &mut ClientSession,
    ) -> error::Result<()> {
        self.companies
            .insert_one_with_session(company, None, session)
            .await?;
        Ok(())
    }

    pub async fn insert_part_with_session(
        &self,
        part: &PartContainer,
        session: &mut ClientSession,
    ) -> error::Result<()> {
        self.parts
            .insert_one_with_session(part, None, session)
            .await?;
        Ok(())
    }

    pub async fn insert_point_with_session(
        &self,
        point: &PointContainer,
        session: &mut ClientSession,
    ) -> error::Result<()> {
        self.points
            .insert_one_with_session(point, None, session)
            .await?;
        Ok(())
    }

    pub async fn find_samples(&self) -> error::Result<Vec<Sample>> {
        Ok(self.samples.find(None, None).await?.try_collect().await?)
    }

    pub async fn find_sample(&self, hash: &str) -> error::Result<Option<Sample>> {
        Ok(self.samples.find_one(doc!("hash": hash), None).await?)
    }

    pub async fn find_package(
        &self,
        sample_hash: &str,
        package_index: usize,
    ) -> error::Result<Option<Package>> {
        match self.find_sample(sample_hash).await? {
            None => Ok(None),
            Some(sample) => {
                let package_index = package_index as u32;
                Ok(self
                    .packages
                    .find_one(doc!("sample_id": sample.id, "index": package_index), None)
                    .await?)
            }
        }
    }

    pub async fn find_company(&self, id: &ObjectId) -> error::Result<Option<Company>> {
        Ok(self.companies.find_one(doc!("_id": id), None).await?)
    }

    pub async fn find_part(&self, id: &ObjectId) -> error::Result<Option<PartContainer>> {
        Ok(self.parts.find_one(doc!("_id": id), None).await?)
    }

    pub async fn find_point(&self, id: &ObjectId) -> error::Result<Option<PointContainer>> {
        Ok(self.points.find_one(doc!("_id": id), None).await?)
    }
}
