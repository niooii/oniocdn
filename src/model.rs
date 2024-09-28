use sqlx::PgPool;


// Types
pub struct Media {
    path: String,

}

pub struct UploadedMediaInfo {
    bytes: Vec<u8>
}

// Model controllers    
#[derive(Clone)]
pub struct MediaController {
    db_pool: PgPool
}

impl MediaController {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool
        }
    }
}

impl MediaController {
    pub async fn save_media(info: UploadedMediaInfo) {
        
    }

    pub async fn get_media( ) {

    }

    pub async fn delete_media() {

    } 
}