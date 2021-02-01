use crate::{task::*, tools};
use tokio::{
    fs::File,
    io::AsyncWriteExt,
};

pub async fn fetch_data(t: content::Task, target: String) -> String {
    let f_name = tools::gen_rand_str(32);
    let path = format!("{}/{}.zip", target, f_name);
    let mut file = File::create(&path)
        .await
        .unwrap();

    let mut res = reqwest::get(&format!("{}/{}.zip", t.test_data_uri, t.pid)).await.unwrap();

    while let Some(chunk) = res.chunk().await.unwrap() {
        file.write(&chunk).await.unwrap();
    }

    path
}
