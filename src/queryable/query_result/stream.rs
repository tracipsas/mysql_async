// Copyright (c) 2019 Thomas Bagrel
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use futures::stream::Stream;
use futures::{try_ready, Async::{self, Ready}};

use crate::{connection_like::ConnectionLike, error::*, queryable::{query_result::QueryResult, Protocol}, Row, BoxFuture};

pub struct RowStream<T, P>
    where
        P: Protocol,
        P: Send + 'static,
        T: ConnectionLike + Sized + 'static
{
    fut: BoxFuture<(QueryResult<T, P>, Option<Row>)>,
}

impl<T, P> RowStream<T, P>
    where
        P: Protocol,
        P: Send + 'static,
        T: ConnectionLike + Sized + 'static
{
    pub fn new(query_result: QueryResult<T, P>) -> RowStream<T, P> {
        RowStream { fut: Box::new(query_result.get_row()) }
    }
}

impl<T, P> Stream for RowStream<T, P>
    where
        P: Protocol,
        P: Send + 'static,
        T: ConnectionLike + Sized + 'static
{
    type Item = Row;
    type Error = Error;

    fn poll(&mut self) -> core::result::Result<Async<Option<Self::Item>>, Self::Error> {
        let (query_result, row_opt) = try_ready!(self.fut.poll());
        self.fut = Box::new(query_result.get_row());
        Ok(Ready(row_opt))
    }
}
