fn main(){
    let mut x = 5;
    println!("{}", x);
    let can_window = WindowsCan{};
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