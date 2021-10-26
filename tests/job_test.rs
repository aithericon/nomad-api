#[cfg(test)]
mod tests {
    use nomad_api::client::NomadClient;
    use std::collections::HashMap;
    use std::path::{Path};

    #[tokio::test]
    async fn list_jobs() {
        let client = NomadClient::new("http://127.0.0.1:4646".to_string(), "".to_string());
        let list_nodes_response = client.list_jobs().await;
        println!("{:?}", &list_nodes_response);
        for job in list_nodes_response.unwrap() {
            println!("Stopping job with id {}", &job.id);
            client.stop_job(&job.id).await.unwrap();
        }
        // list_nodes_response.unwrap();
    }

    #[tokio::test]
    async fn list_job_allocations() {
        let client = NomadClient::new("http://127.0.0.1:4646".to_string(), "".to_string());
        let list_job_allocations_response = client
            .list_job_allocations("numpire-worker-gpu/dispatch-1626361554-dc6a23ac")
            .await;
        println!("{:?}", &list_job_allocations_response);
        list_job_allocations_response.unwrap();
    }

    #[tokio::test]
    async fn read_allocation() {
        let client = NomadClient::new("http://127.0.0.1:4646".to_string(), "".to_string());
        let allocation = client
            .read_allocation("30cd7ef8-cd5c-e950-2ce5-e512fdcdd551")
            .await;
        println!("{:?}", &allocation);
        allocation.unwrap();
    }

    #[tokio::test]
    async fn dispatch_job() {
        let client = NomadClient::new("http://127.0.0.1:4646".to_string(), "".to_string());
        let payload = None;
        let mut meta = HashMap::new();
        meta.insert(
            "numpire_image".to_string(),
            "localhost:5000/agridos-worker".to_string(),
        );
        meta.insert("numpire_node_id".to_string(), "1".to_string());
        let dispatch_result = client
            .dispatch_job("numpire-worker-gpu", payload, meta)
            .await;
        println!("{:?}", &dispatch_result);
        dispatch_result.unwrap();
    }

    #[tokio::test]
    async fn dispatch_job3() {
        let client = NomadClient::new("http://127.0.0.1:4646".to_string(), "".to_string());
        let payload = None;
        let mut meta = HashMap::new();
        meta.insert(
            "numpire_image".to_string(),
            "localhost:5000/numpire-rl-agent".to_string(),
        );
        let dispatch_result = client.dispatch_job("numpire-agent", payload, meta).await;
        println!("{:?}", &dispatch_result);
        dispatch_result.unwrap();
    }

    // #[tokio::test]
    /*async fn dispatch_job2() {
        let client = NomadClient::new("http://127.0.0.1:4646".to_string(), "".to_string());
        let payload = None;
        let mut meta = HashMap::new();
        meta.insert("blink".to_string(), "127.0.0.1:4444".to_string());
        meta.insert("baddr".to_string(), "0x".to_string());
        meta.insert(
            "idle_image".to_string(),
            "ahtonen/docker-ethminer".to_string(),
        );
        let dispatch_result = client.dispatch_job("idle-worker-gpu", payload, meta).await;
        println!("{:?}", &dispatch_result);
        dispatch_result.unwrap();
    }*/

    #[tokio::test]
    async fn parse_job() {
        let client = NomadClient::new("http://127.0.0.1:4646".to_string(), "".to_string());
        let hcl_file =
            std::fs::read_to_string(Path::new("tests/assets/worker-param-gpu.nomad")).unwrap();
        let job = client.parse_job(&hcl_file, true).await;
        println!("{:?}", &job);
        assert!(job.is_ok());
    }

    #[tokio::test]
    async fn create_job() {
        let client = NomadClient::new("http://127.0.0.1:4646".to_string(), "".to_string());
        let hcl_file =
            std::fs::read_to_string(Path::new("tests/assets/worker-param-gpu.nomad")).unwrap();
        let job = client.parse_job(&hcl_file, true).await;
        assert!(job.is_ok());
        let resp = client.create_job(&job.unwrap()).await;

        println!("{:?}", &resp);
        assert!(resp.is_ok());
    }
}
