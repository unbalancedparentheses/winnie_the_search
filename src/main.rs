extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/_nodes", fake_nodes, "nodes");
    router.get("/_search", fake_search, "search");

    fn handler(_: &mut Request) -> IronResult<Response> {
        let banner = r#"{
        "status" : 200,
        "name" : "%s",
        "cluster_name" : "elasticsearch",
        "version" : {
            "number" : "%s",
            "build_hash" : "89d3241d670db65f994242c8e838b169779e2d4",
            "build_snapshot" : false,
            "lucene_version" : "4.10.2"
        },
        "tagline" : "You Know, for Search"
}"#;
        Ok(Response::with((status::Ok, banner)))
    }

    fn fake_nodes(_: &mut Request) -> IronResult<Response> {
        let response = r#"{
        "cluster_name" : "elasticsearch",
        "nodes" : {
            "x1JG6g9PRHy6ClCOO2-C4g" : {
              "name" : "%s",
              "transport_address" : "inet[/
			%s:9300]",
              "host" : "elk",
              "ip" : "127.0.1.1",
              "version" : "%s",
              "build" : "89d3241",
              "http_address" : "inet[/%s:9200]",
              "os" : {
                "refresh_interval_in_millis" : 1000,
                "available_processors" : 12,
                "cpu" : {
                  "total_cores" : 24,
                  "total_sockets" : 48,
                  "cores_per_socket" : 2
                }
              },
              "process" : {
                "refresh_interval_in_millis" : 1000,
                "id" : 2039,
                "max_file_descriptors" : 65535,
                "mlockall" : false
              },
              "jvm" : {
                "version" : "1.7.0_65"
              },
              "network" : {
                "refresh_interval_in_millis" : 5000,
                "primary_interface" : {
                  "address" : "%s",
                  "name" : "eth0",
                  "mac_address" : "08:01:c7:3F:15:DD"
                }
              },
              "transport" : {
                "bound_address" : "inet[/0:0:0:0:0:0:0:0:9300]",
                "publish_address" : "inet[/%s:9300]"
              },
              "http" : {
                "bound_address" : "inet[/0:0:0:0:0:0:0:0:9200]",
                "publish_address" : "inet[/%s:9200]",
                "max_content_length_in_bytes" : 104857600
              }}
            }
}"#;
        Ok(Response::with((status::Ok, response)))
    }

    fn fake_search(_: &mut Request) -> IronResult<Response> {
        let response = r#"{
        "took" : 6,
        "timed_out" : false,
        "_shards" : {
            "total" : 6,
            "successful" : 6,
            "failed" : 0
        },
        "hits" : {
            "total" : 1,
            "max_score" : 1.0,
            "hits" : [ {
                "_index" : ".kibana",
                "_type" : "index-pattern",
                "_id" : "logstash-*",
                "_score" : 1.0,
                "_source": {"title":"logstash-*","timeFieldName":"@timestamp","customFormats":"{}","fields":"[{"type":"string","indexed":true,"analyzed":true,"doc_values":false,"name":"host","count":0},{"type":"string","indexed":false,"analyzed":false,"name":"_source","count":0},{"type":"string","indexed":true,"analyzed":false,"doc_values":false,"name":"message.raw","count":0},{"type":"string","indexed":false,"analyzed":false,"name":"_index","count":0},{"type":"string","indexed":true,"analyzed":false,"doc_values":false,"name":"@version","count":0},{"type":"string","indexed":true,"analyzed":true,"doc_values":false,"name":"message","count":0},{"type":"date","indexed":true,"analyzed":false,"doc_values":false,"name":"@timestamp","count":0},{"type":"string","indexed":true,"analyzed":false,"name":"_type","count":0},{"type":"string","indexed":true,"analyzed":false,"name":"_id","count":0},{"type":"string","indexed":true,"analyzed":false,"doc_values":false,"name":"host.raw","count":0},{"type":"geo_point","indexed":true,"analyzed":false,"doc_values":false,"name":"geoip.location","count":0}]"}
           }]
        }
    }"#;
        
        Ok(Response::with((status::Ok, response)))
    }

    let _server = Iron::new(router).http("localhost:3000").unwrap();
}
