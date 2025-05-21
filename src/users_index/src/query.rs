use candid::Principal;
use errors::user_error::UserError;
use ic_ledger_types::{query_archived_blocks, query_blocks, Block, BlockIndex, GetBlocksArgs};

pub async fn query_one_block(
    ledger: Principal,
    block_index: BlockIndex,
) -> Result<Option<Block>, UserError> {
    let args = GetBlocksArgs {
        start: block_index,
        length: 1,
    };

    let blocks_result = match query_blocks(ledger, args.clone()).await {
        Ok(blocks) => blocks,
        Err(e) => {
            return Err(UserError::QueryError(format!(
                "Error querying blocks: {:?}",
                e
            )))
        }
    };

    if !blocks_result.blocks.is_empty() {
        debug_assert_eq!(blocks_result.first_block_index, block_index);
        return Ok(blocks_result.blocks.into_iter().next());
    }

    if let Some(func) = blocks_result.archived_blocks.into_iter().find_map(|b| {
        (b.start <= block_index && (block_index - b.start) < b.length).then_some(b.callback)
    }) {
        match query_archived_blocks(&func, args).await {
            Ok(blocks) => match blocks {
                Ok(range) => return Ok(range.blocks.into_iter().next()),
                Err(e) => {
                    return Err(UserError::QueryError(format!(
                        "Error querying archived blocks: {:?}",
                        e
                    )))
                }
            },
            Err(e) => {
                return Err(UserError::QueryError(format!(
                    "Error querying archived blocks: {:?}",
                    e
                )))
            }
        }
    }
    Ok(None)
}
