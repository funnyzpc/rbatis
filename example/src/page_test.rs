#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::rbatis::Rbatis;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::crud::CRUD;

    lazy_static! { static ref RB:Rbatis=Rbatis::new();}

    #[async_std::test]
    pub async fn test_sql_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let wraper = rb.new_wrapper()
            .eq("delete_flag", 0).check().unwrap();
        let data: Page<BizActivity> = rb.fetch_page_by_wrapper("", &wraper, &PageRequest::new(1, 20)).await.unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());
    }


    #[async_std::test]
    pub async fn test_py_sql_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let py = r#"
    SELECT * FROM biz_activity
    WHERE delete_flag = #{delete_flag}
    if name != null:
      AND name like #{name+'%'}"#;
        let data: Page<BizActivity> = rb.py_fetch_page("", py, &serde_json::json!({   "delete_flag": 1 }), &PageRequest::new(1, 20)).await.unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());
    }

    #[py_sql(RB, "select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}")]
    async fn py_select_page(page_req: &PageRequest, name: &str) -> Page<BizActivity> {}

    #[async_std::test]
    pub async fn test_macro_py_select_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = py_select_page(&PageRequest::new(1, 10), "test").await.unwrap();
        println!("{:?}", a);
    }
}