#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum State {
    Acting,
    Resting,
    Crystallizing,
    Asleep,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SetState_Data {
    pub flow_id: signal_flow::FlowId,
    pub state: State,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Input {
    SetState(SetState_Data),
    CrystallizeNow(signal_flow::FlowId),
    Renew(flow_ethos::Address),
    Close(signal_flow::FlowId),
    Deliver(flow_ethos::Address),
    Unlock(flow_ethos::Address),
    Forget(signal_flow::FlowId),
    Hold,
    Resume,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StateSet_Data {
    pub flow_id: signal_flow::FlowId,
    pub state: State,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Refused_Data {
    UnknownFlow(signal_flow::FlowId),
    UnknownAddress(flow_ethos::Address),
    Harnessed(signal_flow::FlowId),
    Unharnessed(signal_flow::FlowId),
    Restarting(signal_flow::FlowId),
    HerdrRefused(signal_flow::FlowId),
    Unrenewed(signal_flow::RefreshRejection),
    StoreRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Output {
    StateSet(StateSet_Data),
    Asked(signal_flow::FlowId),
    Renewed(signal_flow::FlowId),
    Closed(signal_flow::FlowId),
    Delivered(flow_ethos::Address),
    Unlocked(flow_ethos::Address),
    Forgotten(signal_flow::FlowId),
    Held,
    Resumed,
    Refused(Refused_Data),
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn set_state(
        &self,
        set_state: &SetState_Data,
    ) -> std::result::Result<StateSet_Data, Output>;
    fn crystallize_now(
        &self,
        flow_id: &signal_flow::FlowId,
    ) -> std::result::Result<signal_flow::FlowId, Output>;
    fn renew(
        &self,
        address: &flow_ethos::Address,
    ) -> std::result::Result<signal_flow::FlowId, Output>;
    fn close(
        &self,
        flow_id: &signal_flow::FlowId,
    ) -> std::result::Result<signal_flow::FlowId, Output>;
    fn deliver(
        &self,
        address: &flow_ethos::Address,
    ) -> std::result::Result<flow_ethos::Address, Output>;
    fn unlock(
        &self,
        address: &flow_ethos::Address,
    ) -> std::result::Result<flow_ethos::Address, Output>;
    fn forget(
        &self,
        flow_id: &signal_flow::FlowId,
    ) -> std::result::Result<signal_flow::FlowId, Output>;
    fn hold(&self) -> std::result::Result<(), Output>;
    fn resume(&self) -> std::result::Result<(), Output>;
}
#[rustfmt::skip]
pub trait Answerable {
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Output;
}
#[rustfmt::skip]
impl Answerable for Input {
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Output {
        match self {
            Input::SetState(carried) => {
                nexus.set_state(carried).map_or_else(|refusal| refusal, Output::StateSet)
            }
            Input::CrystallizeNow(carried) => {
                nexus
                    .crystallize_now(carried)
                    .map_or_else(|refusal| refusal, Output::Asked)
            }
            Input::Renew(carried) => {
                nexus.renew(carried).map_or_else(|refusal| refusal, Output::Renewed)
            }
            Input::Close(carried) => {
                nexus.close(carried).map_or_else(|refusal| refusal, Output::Closed)
            }
            Input::Deliver(carried) => {
                nexus.deliver(carried).map_or_else(|refusal| refusal, Output::Delivered)
            }
            Input::Unlock(carried) => {
                nexus.unlock(carried).map_or_else(|refusal| refusal, Output::Unlocked)
            }
            Input::Forget(carried) => {
                nexus.forget(carried).map_or_else(|refusal| refusal, Output::Forgotten)
            }
            Input::Hold => nexus.hold().map_or_else(|refusal| refusal, |()| Output::Held),
            Input::Resume => {
                nexus.resume().map_or_else(|refusal| refusal, |()| Output::Resumed)
            }
        }
    }
}
