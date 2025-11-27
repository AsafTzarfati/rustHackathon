use prost::Message;
// Re-export the generated protobuf types
use crate::oper_system::api::v1 as proto;

#[derive(Debug, Clone)]
pub enum MessageWrapper {
    SensorBatch(proto::SensorBatch),
    SystemStatus(proto::SystemStatus),
    HardwareStatus(proto::HardwareStatus),
    ClockModulation(proto::ClockModulation),
    TestCase(proto::TestCase),
    SimulationState(proto::SimulationState),
    TestResult(proto::TestResult),
    TimeSync(proto::TimeSync),
    FaultInjection(proto::FaultInjection),
    ActuatorCommand(proto::ActuatorCommand),
    Heartbeat(proto::Heartbeat),
    Ack(proto::Ack),
}

impl MessageWrapper {
    // Unique IDs for each message type
    const ID_SENSOR_BATCH: u8 = 1;
    const ID_SYSTEM_STATUS: u8 = 2;
    const ID_HARDWARE_STATUS: u8 = 3;
    const ID_CLOCK_MODULATION: u8 = 4;
    const ID_TEST_CASE: u8 = 5;
    const ID_SIMULATION_STATE: u8 = 6;
    const ID_TEST_RESULT: u8 = 7;
    const ID_TIME_SYNC: u8 = 8;
    const ID_FAULT_INJECTION: u8 = 9;
    const ID_ACTUATOR_COMMAND: u8 = 10;
    const ID_HEARTBEAT: u8 = 11;
    const ID_ACK: u8 = 12;

    pub fn to_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut buf = Vec::new();
        match self {
            MessageWrapper::SensorBatch(msg) => {
                buf.push(Self::ID_SENSOR_BATCH);
                msg.encode(&mut buf)?;
            }
            MessageWrapper::SystemStatus(msg) => {
                buf.push(Self::ID_SYSTEM_STATUS);
                msg.encode(&mut buf)?;
            }
            MessageWrapper::HardwareStatus(msg) => {
                buf.push(Self::ID_HARDWARE_STATUS);
                msg.encode(&mut buf)?;
            }
            MessageWrapper::ClockModulation(msg) => {
                buf.push(Self::ID_CLOCK_MODULATION);
                msg.encode(&mut buf)?;
            }
            MessageWrapper::TestCase(msg) => {
                buf.push(Self::ID_TEST_CASE);
                msg.encode(&mut buf)?;
            }
            MessageWrapper::SimulationState(msg) => {
                buf.push(Self::ID_SIMULATION_STATE);
                msg.encode(&mut buf)?;
            }
            MessageWrapper::TestResult(msg) => {
                buf.push(Self::ID_TEST_RESULT);
                msg.encode(&mut buf)?;
            }
            MessageWrapper::TimeSync(msg) => {
                buf.push(Self::ID_TIME_SYNC);
                msg.encode(&mut buf)?;
            }
            MessageWrapper::FaultInjection(msg) => {
                buf.push(Self::ID_FAULT_INJECTION);
                msg.encode(&mut buf)?;
            }
            MessageWrapper::ActuatorCommand(msg) => {
                buf.push(Self::ID_ACTUATOR_COMMAND);
                msg.encode(&mut buf)?;
            }
            MessageWrapper::Heartbeat(msg) => {
                buf.push(Self::ID_HEARTBEAT);
                msg.encode(&mut buf)?;
            }
            MessageWrapper::Ack(msg) => {
                buf.push(Self::ID_ACK);
                msg.encode(&mut buf)?;
            }
        }
        Ok(buf)
    }

    pub fn from_bytes(buf: &[u8]) -> Result<Self, prost::DecodeError> {
        if buf.is_empty() {
            return Err(prost::DecodeError::new("Buffer is empty"));
        }

        let id = buf[0];
        let payload = &buf[1..];

        match id {
            Self::ID_SENSOR_BATCH => Ok(MessageWrapper::SensorBatch(proto::SensorBatch::decode(payload)?)),
            Self::ID_SYSTEM_STATUS => Ok(MessageWrapper::SystemStatus(proto::SystemStatus::decode(payload)?)),
            Self::ID_HARDWARE_STATUS => Ok(MessageWrapper::HardwareStatus(proto::HardwareStatus::decode(payload)?)),
            Self::ID_CLOCK_MODULATION => Ok(MessageWrapper::ClockModulation(proto::ClockModulation::decode(payload)?)),
            Self::ID_TEST_CASE => Ok(MessageWrapper::TestCase(proto::TestCase::decode(payload)?)),
            Self::ID_SIMULATION_STATE => Ok(MessageWrapper::SimulationState(proto::SimulationState::decode(payload)?)),
            Self::ID_TEST_RESULT => Ok(MessageWrapper::TestResult(proto::TestResult::decode(payload)?)),
            Self::ID_TIME_SYNC => Ok(MessageWrapper::TimeSync(proto::TimeSync::decode(payload)?)),
            Self::ID_FAULT_INJECTION => Ok(MessageWrapper::FaultInjection(proto::FaultInjection::decode(payload)?)),
            Self::ID_ACTUATOR_COMMAND => Ok(MessageWrapper::ActuatorCommand(proto::ActuatorCommand::decode(payload)?)),
            Self::ID_HEARTBEAT => Ok(MessageWrapper::Heartbeat(proto::Heartbeat::decode(payload)?)),
            Self::ID_ACK => Ok(MessageWrapper::Ack(proto::Ack::decode(payload)?)),
            _ => Err(prost::DecodeError::new(format!("Unknown message ID: {}", id))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let msg = proto::Heartbeat {
            header: None,
            node_id: "test_node".to_string(),
            status: "OK".to_string(),
            uptime_sec: 100,
        };
        let wrapper = MessageWrapper::Heartbeat(msg.clone());
        
        let bytes = wrapper.to_bytes().expect("Failed to encode");
        assert_eq!(bytes[0], MessageWrapper::ID_HEARTBEAT);
        
        let decoded = MessageWrapper::from_bytes(&bytes).expect("Failed to decode");
        
        if let MessageWrapper::Heartbeat(decoded_msg) = decoded {
            assert_eq!(decoded_msg.node_id, msg.node_id);
            assert_eq!(decoded_msg.status, msg.status);
            assert_eq!(decoded_msg.uptime_sec, msg.uptime_sec);
        } else {
            panic!("Wrong message type decoded");
        }
    }
}
