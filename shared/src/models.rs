use prost::Message;
use crate::proto::oper_system::api::v1 as proto_api;

#[derive(Debug, Clone)]
pub enum MessageWrapper {
    SensorBatch(proto_api::SensorBatch),
    SystemStatus(proto_api::SystemStatus),
    HardwareStatus(proto_api::HardwareStatus),
    ClockModulation(proto_api::ClockModulation),
    TestCase(proto_api::TestCase),
    SimulationState(proto_api::SimulationState),
    TestResult(proto_api::TestResult),
    TimeSync(proto_api::TimeSync),
    FaultInjection(proto_api::FaultInjection),
    ActuatorCommand(proto_api::ActuatorCommand),
    Heartbeat(proto_api::Heartbeat),
}

impl MessageWrapper {
    // Simple serialization helper: 1 byte type ID + protobuf payload
    pub fn encode_to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            MessageWrapper::SensorBatch(msg) => {
                buf.push(1);
                msg.encode(&mut buf).unwrap();
            }
            MessageWrapper::SystemStatus(msg) => {
                buf.push(2);
                msg.encode(&mut buf).unwrap();
            }
            MessageWrapper::HardwareStatus(msg) => {
                buf.push(3);
                msg.encode(&mut buf).unwrap();
            }
            MessageWrapper::ClockModulation(msg) => {
                buf.push(4);
                msg.encode(&mut buf).unwrap();
            }
            MessageWrapper::TestCase(msg) => {
                buf.push(5);
                msg.encode(&mut buf).unwrap();
            }
            MessageWrapper::SimulationState(msg) => {
                buf.push(6);
                msg.encode(&mut buf).unwrap();
            }
            MessageWrapper::TestResult(msg) => {
                buf.push(7);
                msg.encode(&mut buf).unwrap();
            }
            MessageWrapper::TimeSync(msg) => {
                buf.push(8);
                msg.encode(&mut buf).unwrap();
            }
            MessageWrapper::FaultInjection(msg) => {
                buf.push(9);
                msg.encode(&mut buf).unwrap();
            }
            MessageWrapper::ActuatorCommand(msg) => {
                buf.push(10);
                msg.encode(&mut buf).unwrap();
            }
            MessageWrapper::Heartbeat(msg) => {
                buf.push(11);
                msg.encode(&mut buf).unwrap();
            }
        }
        buf
    }

    pub fn decode(buf: &[u8]) -> Result<Self, prost::DecodeError> {
        if buf.is_empty() {
            return Err(prost::DecodeError::new("Empty buffer"));
        }
        let type_id = buf[0];
        let payload = &buf[1..];
        match type_id {
            1 => Ok(MessageWrapper::SensorBatch(proto_api::SensorBatch::decode(payload)?)),
            2 => Ok(MessageWrapper::SystemStatus(proto_api::SystemStatus::decode(payload)?)),
            3 => Ok(MessageWrapper::HardwareStatus(proto_api::HardwareStatus::decode(payload)?)),
            4 => Ok(MessageWrapper::ClockModulation(proto_api::ClockModulation::decode(payload)?)),
            5 => Ok(MessageWrapper::TestCase(proto_api::TestCase::decode(payload)?)),
            6 => Ok(MessageWrapper::SimulationState(proto_api::SimulationState::decode(payload)?)),
            7 => Ok(MessageWrapper::TestResult(proto_api::TestResult::decode(payload)?)),
            8 => Ok(MessageWrapper::TimeSync(proto_api::TimeSync::decode(payload)?)),
            9 => Ok(MessageWrapper::FaultInjection(proto_api::FaultInjection::decode(payload)?)),
            10 => Ok(MessageWrapper::ActuatorCommand(proto_api::ActuatorCommand::decode(payload)?)),
            11 => Ok(MessageWrapper::Heartbeat(proto_api::Heartbeat::decode(payload)?)),
            _ => Err(prost::DecodeError::new("Unknown message type")),
        }
    }
}
