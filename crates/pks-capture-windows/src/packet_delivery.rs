#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PacketReadPlan {
    Empty,
    Read { packet_bytes: usize },
    Oversized { required_bytes: Option<usize> },
}

pub(crate) fn plan_packet_read(
    packet_frames: u32,
    channel_count: u8,
    sample_size_bytes: usize,
    buffer_capacity_bytes: usize,
) -> PacketReadPlan {
    if packet_frames == 0 {
        return PacketReadPlan::Empty;
    }
    let required_bytes = (packet_frames as usize)
        .checked_mul(channel_count as usize)
        .and_then(|sample_count| sample_count.checked_mul(sample_size_bytes));
    match required_bytes {
        Some(packet_bytes) if packet_bytes <= buffer_capacity_bytes => {
            PacketReadPlan::Read { packet_bytes }
        }
        required_bytes => PacketReadPlan::Oversized { required_bytes },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_empty_packet_when_planned_then_no_read_is_requested() {
        assert_eq!(plan_packet_read(0, 2, 4, 16_384), PacketReadPlan::Empty);
    }

    #[test]
    fn given_packet_at_capacity_when_planned_then_exact_byte_count_is_retained() {
        assert_eq!(
            plan_packet_read(2_048, 2, 4, 16_384),
            PacketReadPlan::Read {
                packet_bytes: 16_384
            }
        );
    }

    #[test]
    fn given_oversized_packet_when_planned_then_required_byte_count_is_reported() {
        assert_eq!(
            plan_packet_read(2_049, 2, 4, 16_384),
            PacketReadPlan::Oversized {
                required_bytes: Some(16_392)
            }
        );
    }
}
