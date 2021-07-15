use crate::jobs::{Job, ListJobAllocationsResponse, Allocation, DispatchJobRequest, DispatchJobResponse};
use log::{debug, error, info, warn, trace};
use reqwest::Client;
use std::collections::HashMap;

#[derive(Clone)]
pub struct NomadClient {
    http_client: Client,
    base_url: String,
    authorization_token: String,
}

impl NomadClient {
    pub fn new(base_url: String, authorization_token: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            authorization_token,
        }
    }

    /// https://www.nomadproject.io/api-docs/jobs#list-jobs
    /// This endpoint lists all known jobs in the system registered with Nomad.
    /// Method      Path
    /// GET         /v1/jobs
    pub async fn list_jobs(&self) -> Result<Vec<Job>, reqwest::Error> {
        let url = format!("{}/v1/jobs", &self.base_url);
        trace!("ListJobs call to {}", &url);
        let response = self
            .http_client
            .get(&url)
            .header("X-Nomad-Token", &self.authorization_token)
            .send()
            .await?
            .json::<Vec<Job>>()
            .await?;
        Ok(response)
    }


    /// https://www.nomadproject.io/api-docs/jobs#list-job-allocations
    /// Method	Path	                        Produces
    /// GET	    /v1/job/:job_id/allocations	    application/json
    pub async fn list_job_allocations(&self, id: &str) -> Result<Vec<ListJobAllocationsResponse>, reqwest::Error> {
        let url = format!("{}/v1/job/{}/allocations", &self.base_url, id);
        trace!("ListJobAllocations call to {}", &url);
        let response = self
            .http_client
            .get(&url)
            .header("X-Nomad-Token", &self.authorization_token)
            .send()
            .await?
            .json::<Vec<ListJobAllocationsResponse>>()
            .await?;
        Ok(response)
    }

    /// https://www.nomadproject.io/api-docs/jobs#read-allocation
    /// Method	Path	                        Produces
    /// GET	    /v1/allocations/:alloc_id	    application/json
    ///
    /// Parameters:
    /// :alloc_id (string: <required>)- Specifies the UUID of the allocation. This must be the full
    /// UUID, not the short 8-character one. This is specified as part of the path.
    pub async fn read_allocation(&self, id: &str) -> Result<Allocation, reqwest::Error> {
        let url = format!("{}/v1/allocation/{}", &self.base_url, id);
        trace!("ReadAllocation call to {}", &url);
        let response = self
            .http_client
            .get(&url)
            .header("X-Nomad-Token", &self.authorization_token)
            .send()
            .await?
            .json::<Allocation>()
            .await?;
        Ok(response)
    }


    /// https://www.nomadproject.io/api-docs/jobs#dispatch-job
    /// Method	Path	                    Produces
    /// POST	/v1/job/:job_id/dispatch	application/json
    ///
    /// Parameters
    /// :job_id (string: <required>) - Specifies the id of the job (as specified in the job file during submission). This is specified as part of the path.
    ///
    /// Payload (string: "") - Specifies a base64 encoded string containing the payload. This is limited to 16384 bytes (16KiB).
    ///
    /// Meta (meta<string|string>: nil) - Specifies arbitrary metadata to pass to the job.
    pub async fn dispatch_job(&self, job: &str, payload: Option<String>, meta: HashMap<String, String>) -> Result<DispatchJobResponse, reqwest::Error> {
        let url = format!("{}/v1/job/{}/dispatch", &self.base_url, job);
        trace!("Dispatch job call to {}", &url);
        let request = DispatchJobRequest {
            payload: payload.unwrap_or_default(),
            meta,
        };
        let test = serde_json::to_string(&request).unwrap();
        // trace!("{}", test);
        let response = self
            .http_client
            .post(&url)
            .header("X-Nomad-Token", &self.authorization_token)
            .json(&request)
            .send()
            .await?;

        let response = response.json::<DispatchJobResponse>()
           .await?;

        Ok(response)
    }
}
