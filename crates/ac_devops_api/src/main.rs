use ac_git_orchestrator::actions::GitActions;
use ac_aln_integration::aln_integration::AlnIntegration;
use serde::{Deserialize, Serialize};
use warp::Filter;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Deserialize)]
struct ConfigListRequest {
    user_id: String,
    scope: String,
}

#[derive(Debug, Deserialize)]
struct CloneRequest {
    user_id: String,
    repo_url: String,
    autocrlf: Option<bool>,
    depth: Option<u32>,
    single_branch: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct IntegrateRequest {
    user_id: String,
}

#[derive(Debug, Serialize)]
struct ApiError {
    message: String,
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().with_max_level(tracing::Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let redis_url = "redis://localhost:6379";
    let git_actions = GitActions::new(redis_url);

    let git_actions_filter = warp::any().map(move || git_actions.clone());

    let config_list = warp::path!("git" / "config_list")
        .and(warp::post())
        .and(warp::body::json())
        .and(git_actions_filter.clone())
        .and_then(
            |payload: ConfigListRequest, git: GitActions| async move {
                let scope = match payload.scope.as_str() {
                    "all" => ac_aln_rt::model::Scope::All,
                    "system" => ac_aln_rt::model::Scope::System,
                    "global" => ac_aln_rt::model::Scope::Global,
                    "local" => ac_aln_rt::model::Scope::Local,
                    _ => ac_aln_rt::model::Scope::All,
                };
                match git.config_list(&payload.user_id, scope).await {
                    Ok(v) => Ok(warp::reply::json(&v)),
                    Err(e) => {
                        let err = ApiError {
                            message: e.to_string(),
                        };
                        Err(warp::reject::custom(err))
                    }
                }
            },
        );

    let clone_repo = warp::path!("git" / "clone")
        .and(warp::post())
        .and(warp::body::json())
        .and(git_actions_filter.clone())
        .and_then(|payload: CloneRequest, git: GitActions| async move {
            let mut opts = ac_aln_rt::model::CloneOptions::default();
            if let Some(autocrlf) = payload.autocrlf {
                opts.autocrlf = autocrlf;
            }
            opts.depth = payload.depth;
            if let Some(single) = payload.single_branch {
                opts.single_branch = single;
            }
            match git
                .clone_repository(&payload.user_id, &payload.repo_url, opts)
                .await
            {
                Ok(v) => Ok(warp::reply::json(&v)),
                Err(e) => {
                    let err = ApiError {
                        message: e.to_string(),
                    };
                    Err(warp::reject::custom(err))
                }
            }
        });

    let aln_integrate = warp::path!("aln" / "integrate_all")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|payload: IntegrateRequest| async move {
            let res = AlnIntegration::integrate_all(&payload.user_id);
            Ok::<_, warp::Rejection>(warp::reply::json(&res))
        });

    let routes = config_list.or(clone_repo).or(aln_integrate);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
