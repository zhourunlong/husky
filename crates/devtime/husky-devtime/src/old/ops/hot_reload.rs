use crate::*;
use husky_entity_kind::EntityKind;

use std::time::Instant;

impl<Task: IsTask> Devtime<Task> {
    pub fn hot_reload(&mut self) {
        todo!()
        // self.runtime.hot_reload();
        // let old_state = self.clear();
        // self.gen_root_traces();
        // self.mimic_old_state(old_state);
        // self.update();
        // self.take_change()
    }

    fn gen_root_traces(&mut self) {
        todo!()
        // let target_entrance = self.runtime().target_entrance();
        // let now = Instant::now();
        // let main_feature_repr = self.runtime().main_feature_repr(target_entrance);
        // println!(
        //     "{} milliseconds elapsed for computing main feature",
        //     now.elapsed().as_millis(),
        // );
        // let module = self.runtime().module(target_entrance).unwrap();
        // let mut root_traces = vec![];
        // // add input trace
        // root_traces.push(self.new_trace(None, 0, TraceVariant::input(self.runtime())));
        // // add module traces
        // for (subitem_kind, subitem_route) in
        //     self.runtime().subitem_kinded_routes(module).iter()
        // {
        //     match subitem_kind {
        //         EntityKind::Module => {
        //             if self.runtime().module_contains_features(*subitem_route) {
        //                 root_traces.push(self.new_trace(
        //                     None,
        //                     0,
        //                     TraceVariant::Module {
        //                         route: *subitem_route,
        //                         file: target_entrance,
        //                         range: Default::default(),
        //                     },
        //                 ))
        //             }
        //         }
        //         EntityKind::Feature => {
        //             let repr = self.runtime().item_feature_repr(*subitem_route);
        //             root_traces.push(self.new_trace(
        //                 None,
        //                 0,
        //                 TraceVariant::EntityFeature {
        //                     route: *subitem_route,
        //                     repr,
        //                 },
        //             ))
        //         }
        //         _ => (),
        //     }
        // }
        // // add main trace
        // root_traces.push(self.new_trace(None, 0, TraceVariant::Main(main_feature_repr)));
        // self.state.set_root_traces(root_traces);
    }

    fn mimic_old_state(&mut self, mut old_state: DevtimeOldState) {
        // order matters
        self.mimic_old_expansions(&mut old_state);
        old_state.fix();
        self.mimic_old_presentation(&old_state)
    }

    fn mimic_old_expansions(&mut self, old_state: &mut DevtimeOldState) {
        self.mimic_old_expansions_dfs(0, old_state)
    }

    fn mimic_old_expansions_dfs(&mut self, start: usize, old_state: &mut DevtimeOldState) {
        let end = self.state.trace_nodes.len();
        if start >= end {
            return;
        }
        for idx in start..end {
            let trace_node = &self.state.trace_nodes[idx];
            if let Some(old_trace_node) = old_state.try_match_node(trace_node) {
                if old_trace_node.expanded() != trace_node.expanded() {
                    let new_trace_id = trace_node.trace().id();
                    self.state
                        .trace_nodes
                        .update_elem(idx, |node| node.toggle_expansion());
                    self.update_subtraces(new_trace_id)
                }
            }
        }
        self.mimic_old_expansions_dfs(end, old_state)
    }

    fn mimic_old_presentation(&mut self, old_state: &DevtimeOldState) {
        self.state
            .set_presentation(old_state.mimic_presentation(&self.state.trace_nodes))
    }
}