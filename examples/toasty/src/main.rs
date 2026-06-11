//! Toasty — one model definition, multiple backends.
//!
//! This shows the part of Toasty that is stable and well-documented: defining a
//! model with the derive macro and expressing queries with the generated builder
//! DSL (projections via `.select()`, scalar `Vec` fields, filters).
//!
//! The `Db` connection/bootstrap API is the fastest-moving surface in 0.6, so the
//! actual wiring is left to the official guide rather than guessed at here:
//!   https://tokio-rs.github.io/toasty/nightly/guide/
//!
//! Verdict from the Deep Dive: ASSESS — bet on it for greenfield + SQL/DynamoDB,
//! re-evaluate for production at 1.0.

#[derive(Debug, toasty::Model)]
struct Article {
    #[key]
    #[auto]
    id: u64,

    title: String,

    // Stored using the target DB's best fit: Postgres arrays, JSON elsewhere,
    // native lists on DynamoDB.
    tags: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Connect (see the guide for the 0.6 Db bootstrap against SQLite/Postgres/DynamoDB):
    //    let mut db = /* toasty::Db::builder()... */;

    // 2) Create — note the typed builder generated from the model:
    //    Article::create()
    //        .title("hello toasty!")
    //        .tags(["announcement", "orm"])
    //        .exec(&mut db)
    //        .await?;

    // 3) Query with a projection so you only load what you need
    //    (the "don't drag the full body into the index page" case):
    //    let titles: Vec<String> = Article::filter_by_id(1)
    //        .select(Article::fields().title())
    //        .exec(&mut db)
    //        .await?;

    // 4) Array-aware filter on a Vec<scalar> field:
    //    let related = Article::filter(
    //        Article::fields().tags().intersects(["rust", "toasty"]),
    //    )
    //    .exec(&mut db)
    //    .await?;

    println!("Model `Article` compiles against the Toasty derive macro.");
    println!("Wire up `Db` per the guide, then uncomment the create/query flow.");
    Ok(())
}
