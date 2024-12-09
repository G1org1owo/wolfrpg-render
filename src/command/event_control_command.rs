use serde::Serialize;
use crate::command::event_control_command::erase_event::EraseEvent;
use crate::command::event_control_command::loop_command::Loop;
use crate::command::event_control_command::move_route::MoveRoute;
use crate::command::event_control_command::set_transition::SetTransition;
use crate::command::event_control_command::wait::Wait;

mod loop_command;
mod set_transition;
mod move_route;
mod erase_event;
mod wait;

const COMMAND_END_SIGNATURE_LENGTH: usize = 3;

#[derive(Serialize)]
pub enum EventControlCommand {
    Loop(Loop),
    BreakLoop,
    GotoLoopStart,
    PrepareTransition,
    ExecuteTransition,
    SetTransition(SetTransition),
    MoveRoute(MoveRoute),
    WaitForMoveRoute,
    MoveDuringEventsOn,
    MoveDuringEventsOff,
    GotoTitle,
    GameEnd,
    StopNonPictureGraphicUpdates,
    ResumeNonPictureGraphicUpdates,
    ForceExitEvent,
    EraseEvent(EraseEvent),
    Wait(Wait),
    LoopForCount,
    LabelPoint,
    LabelJump
}

impl EventControlCommand {
    fn parse_empty_command(command: EventControlCommand) -> (usize, Self) {
        (COMMAND_END_SIGNATURE_LENGTH, command)
    }
    pub fn parse_loop(bytes: &[u8]) -> (usize, u32, Self) {
        let (bytes_read, commands_read, command): (usize, u32, Loop) = Loop::parse(bytes);

        (bytes_read, commands_read, Self::Loop(command))
    }

    pub fn parse_break_loop(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::BreakLoop)
    }

    pub fn parse_goto_loop_start(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::GotoLoopStart)
    }

    pub fn parse_prepare_transition(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::PrepareTransition)
    }

    pub fn parse_execute_transition(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::ExecuteTransition)
    }

    pub fn parse_set_transition(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, SetTransition) = SetTransition::parse(bytes);

        (bytes_read, Self::SetTransition(command))
    }

    pub fn parse_move_route(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, MoveRoute) = MoveRoute::parse(bytes);

        (bytes_read, Self::MoveRoute(command))
    }

    pub fn parse_wait_for_move_route(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::WaitForMoveRoute)
    }

    pub fn parse_move_during_events_on(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::MoveDuringEventsOn)
    }

    pub fn parse_move_during_events_off(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::MoveDuringEventsOff)
    }

    pub fn parse_goto_title(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::GotoTitle)
    }

    pub fn parse_game_end(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::GameEnd)
    }

    pub fn parse_stop_non_picture_graphic_updates(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::StopNonPictureGraphicUpdates)
    }

    pub fn parse_resume_non_picture_graphic_updates(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::ResumeNonPictureGraphicUpdates)
    }

    pub fn parse_force_exit_event(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::ForceExitEvent)
    }

    pub fn parse_erase_event(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, EraseEvent) = EraseEvent::parse(bytes);

        (bytes_read, Self::EraseEvent(command))
    }

    pub fn parse_wait(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Wait) = Wait::parse(bytes);

        (bytes_read, Self::Wait(command))
    }
}