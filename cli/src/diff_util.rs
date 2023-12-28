use futures::{try_join, Stream, StreamExt};
use jj_lib::backend::{BackendResult, ObjectId, TreeValue};
use jj_lib::repo_path::{RepoPath, RepoPathBuf};
use jj_lib::store::Store;
    tree_diff: TreeDiffStream,
    let mut diff_stream = materialized_diff_stream(workspace_command.repo().store(), tree_diff);
        while let Some((path, diff)) = diff_stream.next().await {
fn materialized_diff_stream<'a>(
    store: &'a Store,
    tree_diff: TreeDiffStream<'a>,
) -> impl Stream<
    Item = (
        RepoPathBuf,
        BackendResult<(MaterializedTreeValue, MaterializedTreeValue)>,
    ),
> + 'a {
    tree_diff
        .map(|(path, diff)| async {
            match diff {
                Err(err) => (path, Err(err)),
                Ok((before, after)) => {
                    let before_future = materialize_tree_value(store, &path, before);
                    let after_future = materialize_tree_value(store, &path, after);
                    let values = try_join!(before_future, after_future);
                    (path, values)
                }
            }
        })
        .buffered((store.concurrency() / 2).max(1))
}

    tree_diff: TreeDiffStream,

    let mut diff_stream = materialized_diff_stream(workspace_command.repo().store(), tree_diff);
        while let Some((path, diff)) = diff_stream.next().await {
    tree_diff: TreeDiffStream,
    let mut diff_stream = materialized_diff_stream(workspace_command.repo().store(), tree_diff);
        while let Some((repo_path, diff)) = diff_stream.next().await {