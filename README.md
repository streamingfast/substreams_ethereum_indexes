# substreams_ethereum_indexes

This substream, named `substreams_ethereum_indexes`, includes the `block_index` module which extracts all events and calls from Ethereum's complete chain blocks.

### Output Data

The `block_index` module extracts events and calls from Ethereum blocks, categorizing them under specific prefixes and storing them as keys of `proto:sf.substreams.index.v1.keys` type.

- **Event Signature**: Event signatures are stored with the prefix `es`. For example:
    ```
    es:ea0f544916910bb1ff33390cbe54a3f5d36d298328578399311cde3c9a750686
    ```

- **Event Contract Address**: Event addresses are stored with the prefix `ea`. For example:
    ```
    ea:276c5c6ca8507ed7bac085fc9b9521f4f54b58d3
    ```

- **Call Method**: Call methods are stored with the prefix `ca`. For example:
    ```
    ca:1158c3c9a70e85d8358972810ed984c8e6ffcf0f
    ```

- **Call Contract Address**: Call addresses are stored with the prefix `cm`. For example:
    ```
    cm:f8b2cb4f
    ```

