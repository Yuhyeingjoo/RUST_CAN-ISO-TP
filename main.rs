fn main(){
    let mut x = 5;
    let array : [u8; 5] = [1, 2, 3, 4, 5];
    println!("{}", x);
    let can_window = WindowsCan{};
    can_window.send(1, &array);
}


pub trait CanInterface {
    /// CAN 메시지 전송
    fn send(&self, id: u32, data: &[u8]) -> Result<(), CanError>;
    
    /// CAN 메시지 수신
    fn receive(&self, id: u32) -> Result<Vec<u8>, CanError>;
}
pub struct WindowsCan;

impl CanInterface for WindowsCan {
    fn send(&self, id: u32, data: &[u8]) -> Result<(), CanError> {
        // 실제 Windows 환경에서 CAN 통신을 구현
        // 예시로는 Windows에서 제공하는 라이브러리나 API 호출 등을 사용할 수 있습니다.
        println!("Windows CAN: Sending data with ID {}: {:?}", id, data);
        Ok(())
    }

    fn receive(&self, id: u32) -> Result<Vec<u8>, CanError> {
        // 실제 수신 코드
        println!("Windows CAN: Receiving data with ID {}", id);
        Ok(vec![0x01, 0x02, 0x03])  // 예시 데이터
    }
}

#[derive(Debug)]
pub enum CanError {
    SendError(String),
    ReceiveError(String),
    OtherError(String),
}

#[derive(Debug, PartialEq)]
pub enum IsoTpFrame {
    SingleFrame { length: u8, data: [u8; 7] },
    FirstFrame { length: u16, data: [u8; 6] },
    ConsecutiveFrame { sequence_number: u8, data: [u8; 7] },
    FlowControl { flow_status: FlowStatus, block_size: u8, st_min: u8 },
}

#[derive(Debug, PartialEq)]
pub enum FlowStatus {
    ContinueToSend,
    Wait,
    Overflow,
}
impl IsoTpFrame {
    /// Parse a CAN message payload into an ISO-TP frame.
    pub fn parse(payload: &[u8]) -> Result<IsoTpFrame, &'static str> {
        if payload.len() != 8 {
            return Err("Invalid payload length");
        }

        let pci = payload[0] >> 4; // Protocol Control Information (PCI)
        match pci {
            0x0 => Ok(IsoTpFrame::SingleFrame {
                length: payload[0] & 0x0F,
                data: payload[1..8].try_into().unwrap_or([0; 7]),
            }),
            0x1 => Ok(IsoTpFrame::FirstFrame {
                length: ((payload[0] as u16 & 0x0F) << 8) | payload[1] as u16,
                data: payload[2..8].try_into().unwrap_or([0; 6]),
            }),
            0x2 => Ok(IsoTpFrame::ConsecutiveFrame {
                sequence_number: payload[0] & 0x0F,
                data: payload[1..8].try_into().unwrap_or([0; 7]),
            }),
            0x3 => Ok(IsoTpFrame::FlowControl {
                flow_status: match payload[0] & 0x0F {
                    0x0 => FlowStatus::ContinueToSend,
                    0x1 => FlowStatus::Wait,
                    0x2 => FlowStatus::Overflow,
                    _ => return Err("Invalid flow status"),
                },
                block_size: payload[1],
                st_min: payload[2],
            }),
            _ => Err("Unknown PCI"),
        }
    }
}

impl IsoTpFrame {
    /// Create a CAN message payload from an ISO-TP frame.
    pub fn to_payload(&self) -> [u8; 8] {
        match self {
            IsoTpFrame::SingleFrame { length, data } => {
                let mut payload = [0u8; 8];
                payload[0] = (*length & 0x0F) | 0x00;
                payload[1..8].copy_from_slice(data);
                payload
            }
            IsoTpFrame::FirstFrame { length, data } => {
                let mut payload = [0u8; 8];
                payload[0] = ((*length >> 8) as u8 & 0x0F) | 0x10;
                payload[1] = *length as u8;
                payload[2..8].copy_from_slice(data);
                payload
            }
            IsoTpFrame::ConsecutiveFrame {
                sequence_number,
                data,
            } => {
                let mut payload = [0u8; 8];
                payload[0] = (*sequence_number & 0x0F) | 0x20;
                payload[1..8].copy_from_slice(data);
                payload
            }
            IsoTpFrame::FlowControl {
                flow_status,
                block_size,
                st_min,
            } => {
                let mut payload = [0u8; 8];
                payload[0] = match flow_status {
                    FlowStatus::ContinueToSend => 0x30,
                    FlowStatus::Wait => 0x31,
                    FlowStatus::Overflow => 0x32,
                };
                payload[1] = *block_size;
                payload[2] = *st_min;
                payload
            }
        }
    }
}