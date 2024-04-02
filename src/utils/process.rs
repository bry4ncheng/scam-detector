use crate::domain::models::{ScamLLM};

pub fn process_scam(res: Vec<Vec<ScamLLM>>) -> (bool, String) {
    let mut is_scam = false;
    let mut accuracy_scam_counter = 0;

    for resp in res {
        let mut label_0 = 0.0;
        let mut label_1 = 0.0;
        //Confidence of no scam
        for label in resp {
            if label.label == "LABEL_1" {
                label_1 = label.score
            } else {
                label_0 = label.score
            }
        }
        if label_0 < 0.5  {
            accuracy_scam_counter += 1;
        }
    }

    if accuracy_scam_counter == 3 {
        is_scam = true;
        (is_scam, "Very Confident".to_string())
    } else if accuracy_scam_counter == 2 {
        is_scam = true;
        (is_scam, "Confident".to_string())
    } else if accuracy_scam_counter == 1 {
        (is_scam, "Confident".to_string())
    } else {
        (is_scam, "Very Confident".to_string())
    }
}