use axum::extract::FromRef;
use tracing::info;
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
        info!("LLM result: {:?}", res);
        let mut label_0 = 0.0;
        let mut label_1 = 0.0;
        //Confidence of no scam
        for label in res {
            if label.label == "LABEL_1" {
                label_1 = label.score
            } else {
                label_0 = label.score
            }
        }
        //Confidence of scam
        //TODO: Check if correct logic
        ///Ill keep label 1 in in case yall wanna change the response
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