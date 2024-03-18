use axum::extract::FromRef;
use crate::domain::models::BotDistriktWebhookResponse;
use crate::error::AppResult;
use crate::repository::hugging_face_repo::HuggingFaceRepository;
use crate::service::service_registry::ServiceRegistry;

#[derive(Clone)]
pub struct HuggingFaceService {
    hugging_face_repository: HuggingFaceRepository
}

impl FromRef<ServiceRegistry> for HuggingFaceService {
    fn from_ref(state: &ServiceRegistry) -> Self { state.hf_service.clone().unwrap() }
}


impl HuggingFaceService {
    pub fn new(
        hugging_face_repository: HuggingFaceRepository
    ) -> Self {
        Self { hugging_face_repository }
    }

    pub async fn get_phishing_scam(self, msg: String) -> AppResult<BotDistriktWebhookResponse> {
        let res = self
            .hugging_face_repository
            .use_phishbot(msg)
            .await?;
        //Confidence of no scam
        let label_0 = res[0].clone().score;
        //Confidence of scam
        // let label_1 = res[1].clone();
        //TODO: Check if correct logic
        if label_0 > 0.5  {
            Ok(BotDistriktWebhookResponse {
                responses: vec![
                    "This is unlikely a scam!".to_string()
                ],
            })
        } else {
            Ok(BotDistriktWebhookResponse {
                responses: vec![
                    "This is probably a scam! Please do not click on any links or reply to any messages.".to_string()
                ],
            })
        }
    }
}