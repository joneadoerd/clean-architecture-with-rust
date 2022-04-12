use crate::{
    model::view::json::{Error, Response, Result},
    presenter::Present,
};
use http::StatusCode;
use std::convert::TryFrom;

#[derive(Default, Clone)]
pub struct Presenter;

mod thought {
    use super::*;
    use crate::model::{
        app::thought as app,
        view::json::{area_of_life::AreaOfLifeId, thought as view},
    };

    // -- Create -- //

    impl Present<app::create::Result> for Presenter {
        type ViewModel = Result<view::ThoughtId, view::create::Error>;
        fn present(&self, res: app::create::Result) -> Self::ViewModel {
            res.map(view::ThoughtId::from)
                .map(|id| Response {
                    data: Some(id),
                    status: StatusCode::CREATED,
                })
                .map_err(|err| {
                    use app::create::Error as E;
                    match err {
                        E::AreaOfLifeId => Error {
                            msg: Some(err.to_string()),
                            status: StatusCode::BAD_REQUEST,
                            details: Some(view::create::Error::AreaOfLifeId),
                        },
                        E::Invalidity(invalidity) => Error {
                            msg: Some(invalidity.to_string()),
                            status: StatusCode::BAD_REQUEST,
                            details: Some(view::create::Error::from(invalidity)),
                        },
                        E::AreasOfLifeNotFound(ref ids) => Error {
                            msg: Some(err.to_string()),
                            status: StatusCode::BAD_REQUEST,
                            details: Some(view::create::Error::AreasOfLifeNotFound(
                                ids.clone().into_iter().map(AreaOfLifeId::from).collect(),
                            )),
                        },
                        E::Repo | E::NewId => Error::internal(),
                    }
                })
        }
    }

    // -- Update -- //

    impl Present<app::update::Result> for Presenter {
        type ViewModel = Result<(), view::update::Error>;
        fn present(&self, res: app::update::Result) -> Self::ViewModel {
            res.map(|_| Response {
                data: None,
                status: StatusCode::OK,
            })
            .map_err(|err| {
                use app::update::Error as E;
                match err {
                    E::Id => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(view::update::Error::Id),
                    },
                    E::NotFound(id) => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::NOT_FOUND,
                        details: Some(view::update::Error::NotFound(id.into())),
                    },
                    E::AreaOfLifeId => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(view::update::Error::AreaOfLifeId),
                    },
                    E::Invalidity(invalidity) => Error {
                        msg: Some(invalidity.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(view::update::Error::from(invalidity)),
                    },
                    E::AreasOfLifeNotFound(ref ids) => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(view::update::Error::AreasOfLifeNotFound(
                            ids.clone().into_iter().map(AreaOfLifeId::from).collect(),
                        )),
                    },
                    E::Repo => Error::internal(),
                }
            })
        }
    }

    // -- Find by ID -- //

    impl Present<app::find_by_id::Result> for Presenter {
        type ViewModel = Result<view::Thought, view::find_by_id::Error>;
        fn present(&self, res: app::find_by_id::Result) -> Self::ViewModel {
            res.map(view::Thought::from)
                .map(|data| Response {
                    data: Some(data),
                    status: StatusCode::OK,
                })
                .map_err(|err| match err {
                    app::find_by_id::Error::Id => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(view::find_by_id::Error::Id),
                    },
                    app::find_by_id::Error::NotFound => Error {
                        msg: Some("Could not find thought".to_string()),
                        status: StatusCode::NOT_FOUND,
                        details: Some(view::find_by_id::Error::NotFound),
                    },
                    app::find_by_id::Error::Repo => Error::internal(),
                })
        }
    }

    // -- Read all -- //

    impl Present<app::read_all::Result> for Presenter {
        type ViewModel = Result<Vec<view::Thought>, view::read_all::Error>;
        fn present(&self, res: app::read_all::Result) -> Self::ViewModel {
            res.map(|resp| resp.thoughts.into_iter().map(view::Thought::from).collect())
                .map(|data| Response {
                    data: Some(data),
                    status: StatusCode::OK,
                })
                .map_err(|err| match err {
                    app::read_all::Error::Repo => Error::internal(),
                })
        }
    }

    // -- Delete by ID -- //

    impl Present<app::delete::Result> for Presenter {
        type ViewModel = Result<(), view::delete::Error>;
        fn present(&self, res: app::delete::Result) -> Self::ViewModel {
            res.map(|_| Response {
                data: None,
                status: StatusCode::OK,
            })
            .map_err(|err| match err {
                app::delete::Error::Id => Error {
                    msg: Some(err.to_string()),
                    status: StatusCode::BAD_REQUEST,
                    details: Some(view::delete::Error::Id),
                },
                app::delete::Error::NotFound => Error {
                    msg: Some("Could not find thought".to_string()),
                    status: StatusCode::NOT_FOUND,
                    details: Some(view::delete::Error::NotFound),
                },
                app::delete::Error::Repo => Error::internal(),
            })
        }
    }
}

mod area_of_life {
    use super::*;
    use crate::model::{app::area_of_life as app, view::json::area_of_life as view};

    // -- Create -- //

    impl Present<app::create::Result> for Presenter {
        type ViewModel = Result<view::AreaOfLifeId, view::create::Error>;
        fn present(&self, res: app::create::Result) -> Self::ViewModel {
            res.map(view::AreaOfLifeId::from)
                .map(|id| Response {
                    data: Some(id),
                    status: StatusCode::CREATED,
                })
                .map_err(|err| {
                    use app::create::Error as E;
                    match &err {
                        E::Invalidity(invalidity) => Error {
                            msg: Some(invalidity.to_string()),
                            status: StatusCode::BAD_REQUEST,
                            details: view::create::Error::try_from(err).ok(),
                        },
                        E::Repo | E::NewId => Error::internal(),
                    }
                })
        }
    }

    // -- Update -- //

    impl Present<app::update::Result> for Presenter {
        type ViewModel = Result<(), view::update::Error>;
        fn present(&self, res: app::update::Result) -> Self::ViewModel {
            res.map(|_| Response {
                data: None,
                status: StatusCode::OK,
            })
            .map_err(|err| {
                use app::update::Error as E;
                match err {
                    E::Id => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(view::update::Error::Id),
                    },
                    E::NotFound(_) => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::NOT_FOUND,
                        details: Some(view::update::Error::NotFound),
                    },
                    E::Invalidity(invalidity) => Error {
                        msg: Some(invalidity.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(view::update::Error::from(invalidity)),
                    },
                    E::Repo => Error::internal(),
                }
            })
        }
    }

    // -- Read all -- //

    impl Present<app::read_all::Result> for Presenter {
        type ViewModel = Result<Vec<view::AreaOfLife>, view::read_all::Error>;
        fn present(&self, res: app::read_all::Result) -> Self::ViewModel {
            res.map(|resp| {
                resp.areas_of_life
                    .into_iter()
                    .map(view::AreaOfLife::from)
                    .collect()
            })
            .map(|data| Response {
                data: Some(data),
                status: StatusCode::OK,
            })
            .map_err(|err| match err {
                app::read_all::Error::Repo => Error::internal(),
            })
        }
    }

    // -- Delete by ID -- //

    impl Present<app::delete::Result> for Presenter {
        type ViewModel = Result<(), view::delete::Error>;
        fn present(&self, res: app::delete::Result) -> Self::ViewModel {
            res.map(|_| Response {
                data: None,
                status: StatusCode::OK,
            })
            .map_err(|err| match err {
                app::delete::Error::Id => Error {
                    msg: Some(err.to_string()),
                    status: StatusCode::BAD_REQUEST,
                    details: Some(view::delete::Error::Id),
                },
                app::delete::Error::NotFound => Error {
                    msg: Some("Could not find area of life".to_string()),
                    status: StatusCode::NOT_FOUND,
                    details: Some(view::delete::Error::NotFound),
                },
                app::delete::Error::Repo => Error::internal(),
            })
        }
    }
}
