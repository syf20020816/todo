use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Annex {
    r#type: String,
    data: String,
}

impl From<&str> for Annex {
    fn from(value: &str) -> Self {
        let cut: Vec<&str> = value.split("base64,").collect();

        Annex {
            r#type: cut[0].to_string(),
            data: cut[1].to_string(),
        }
    }
}

#[cfg(test)]
mod test_annex {
    use super::Annex;

    #[test]
    fn from() {
        let data ="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMgAAADICAYAAACtWK6eAAALhUlEQVR4Xu3dgXXbRhLGcaeCxBWcXEHsDqQO4g7iDpIKLFeQdBB1YF0F0lUQp4LwKojSQWbugTpaEsnF7rczAPaP9/jklwCDxTf4eUkZAL95xUICJHA0gW/IhgRI4HgCAFnI2XF3d/edDeXtNJwvV1dXDwsZ2tDDAEhy+w2Go/jtAMd+RF/sDx8Miv9kSUoAIEnB+24Nx0";
        let annex: Annex = data.into();
        dbg!(annex);
    }
}
