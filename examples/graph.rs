extern crate differential_aggregate_query;
extern crate differential_dataflow;
extern crate timely;

use differential_dataflow::input::InputSession;

use differential_aggregate_query::factors::graph::GraphFactor;

use differential_aggregate_query::semiring::max_prod::MaxProd;
use differential_aggregate_query::{Factor, InsideOut, Query, Value};

fn main() {
    timely::execute_from_args(std::env::args(), move |worker| {
        let mut input = InputSession::new();

        // Triangle counting
        worker.dataflow::<u64, _, _>(|scope| {
            let graph_factor_1 = GraphFactor {
                variables: vec![2, 3],
                tuples: input.to_collection(scope),
            };
            let graph_factor_2 = GraphFactor {
                variables: vec![1, 2],
                tuples: input.to_collection(scope),
            };
            let graph_factor_3 = GraphFactor {
                variables: vec![1, 3],
                tuples: input.to_collection(scope),
            };

            let faq = Query {
                factors: vec![graph_factor_1, graph_factor_2, graph_factor_3],
                variable_order: vec![3, 2, 1],
            };

            // Run insideOut on out FAQ query
            let output = faq.inside_out();

            //Examin the output
            output.tuples().inspect(|x| println!("{:?}", x));
        });

        // Create a few edges
        input.advance_to(0);
        input.update(
            vec![Value::Number(1), Value::Number(2)],
            MaxProd { value: 1 },
        );
        input.update(
            vec![Value::Number(1), Value::Number(3)],
            MaxProd { value: 1 },
        );
        input.update(
            vec![Value::Number(2), Value::Number(3)],
            MaxProd { value: 1 },
        );
        input.update(
            vec![Value::Number(2), Value::Number(4)],
            MaxProd { value: 1 },
        );
        input.update(
            vec![Value::Number(4), Value::Number(5)],
            MaxProd { value: 1 },
        );
        input.update(
            vec![Value::Number(5), Value::Number(6)],
            MaxProd { value: 1 },
        );
        input.update(
            vec![Value::Number(6), Value::Number(7)],
            MaxProd { value: 1 },
        );
        input.advance_to(1);
        input.update(
            vec![Value::Number(5), Value::Number(7)],
            MaxProd { value: 1 },
        );
    })
    .expect("Computation terminated abnormally");
}
