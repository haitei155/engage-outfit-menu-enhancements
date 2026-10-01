use unity::prelude::*;

// UnityEngine.AnimationEvent$$get_stringParameter	7103ead6c0	System_String_o * UnityEngine.AnimationEvent$$get_stringParameter(UnityEngine_AnimationEvent_o * __this, MethodInfo * method)	8
#[unity::from_offset("UnityEngine", "AnimationEvent", "get_stringParameter")]
pub fn animationevent_get_stringparameter(
    this: *const u8,
    method_info: OptionalMethod,
) -> Option<&'static Il2CppString>;
