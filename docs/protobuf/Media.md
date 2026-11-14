# Media

_Generated from `com.igpsport.blelib.bean.Media`._

## Enums

### `MEDIA_STATUS`

| Name | Value |
|---|---:|
| `INVALID_STATUS` | 0 |
| `NO_ACCESS_STATUS` | 1 |

### `PLAYER_STATE`

| Name | Value |
|---|---:|
| `PAUSED` | 0 |
| `PLAYING` | 1 |
| `REWINDING` | 2 |
| `FORWARDING` | 3 |

### `QUENE_PLAY_MODE`

| Name | Value |
|---|---:|
| `ONE_REPEAT` | 0 |
| `ALL_REPEAT` | 1 |
| `SHUFFLE` | 2 |

### `REMOTE_CMD`

| Name | Value |
|---|---:|
| `PLAY` | 0 |
| `PAUSE` | 1 |
| `TOGGLE_PLAY_PAUSE` | 2 |
| `NEXT_TRACK` | 3 |
| `PREVIOUS_TRACK` | 4 |
| `VOLUME_UP` | 5 |
| `VOLUME_DOWN` | 6 |

## Messages

### `media_message`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `operate_type` | `Common.SERVICE_OPERATE_TYPE` |
| 3 | `player_msg` | `player_message` |
| 4 | `queue_msg` | `queue_message` |
| 5 | `track_msg` | `track_message` |
| 6 | `status` | `MEDIA_STATUS` |

### `player_message`

| # | Field | Type |
|---:|---|---|
| 1 | `player_name` | `string` |
| 2 | `player_state` | `PLAYER_STATE` |
| 3 | `player_rate` | `int32` |
| 4 | `elapsed_time` | `int32` |
| 5 | `volume_max` | `int32` |
| 6 | `volume_cur` | `int32` |

### `queue_message`

| # | Field | Type |
|---:|---|---|
| 1 | `queue_index` | `int32` |
| 2 | `queue_count` | `int32` |
| 3 | `mode` | `QUENE_PLAY_MODE` |

### `track_message`

| # | Field | Type |
|---:|---|---|
| 1 | `artist` | `string` |
| 2 | `album` | `string` |
| 3 | `title` | `string` |
| 4 | `total_time` | `int32` |
