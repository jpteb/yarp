use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use metrics::counter;
use metrics_exporter_prometheus::PrometheusBuilder;
use pingora::prelude::*;

pub struct LB(Arc<LoadBalancer<RoundRobin>>);
pub struct LoadBalancerContext {
    upstream: String,
}

#[async_trait]
impl ProxyHttp for LB {
    #[doc = " The per request object to share state across the different filters"]
    type CTX = LoadBalancerContext;

    #[doc = " Define how the `ctx` should be created."]
    fn new_ctx(&self) -> Self::CTX {
        Self::CTX {
            upstream: String::new(),
        }
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let upstream = self.0.select(b"", 256).unwrap();

        ctx.upstream = upstream.to_string();
        let peer = Box::new(HttpPeer::new(upstream, true, "one.one.one.one".to_string()));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_request
            .insert_header("Host", "one.one.one.one")
            .unwrap();
        Ok(())
    }

    async fn logging(&self, session: &mut Session, _error: Option<&Error>, ctx: &mut Self::CTX) {
        println!("{}{}", ctx.upstream, session.req_header().uri);
        counter!("requests_total").increment(1);
    }
}

// #[tokio::main]
fn main() {
    println!("Hello, world!");
    PrometheusBuilder::new()
        .with_http_listener(([0, 0, 0, 0], 9000))
        .install()
        .expect("failed to install prometheus exporter");

    let mut server = Server::new(Some(Opt::parse_args())).unwrap();
    server.bootstrap();

    let mut upstreams =
        LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443", "127.0.0.1:343"]).unwrap();

    let health_check = TcpHealthCheck::new();
    upstreams.set_health_check(health_check);
    upstreams.health_check_frequency = Some(Duration::from_secs(1));

    let background = background_service("health check", upstreams);
    let upstreams = background.task();

    let mut lb = http_proxy_service(&server.configuration, LB(upstreams));
    lb.add_tcp("0.0.0.0:6188");

    server.add_service(background);

    server.add_service(lb);
    server.run_forever();
}
