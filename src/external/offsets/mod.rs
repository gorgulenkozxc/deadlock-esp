#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod client {
    pub static mut dwEntityList: usize = 0x0;
    pub static mut dwViewMatrix: usize = 0x0;
    pub static mut dwLocalPlayerController: usize = 0x0;
    pub static mut dwCCitadelCameraManager: usize = 0x0;
    pub static mut dwGlobalVars: usize = 0x0;
    pub static mut dwGameRules: usize = 0x0;
    pub static mut highestEntityIndex: usize = 0x2100;
}

pub mod client_dll {
    pub mod CBasePlayerController {
        pub const m_CommandContext: usize = 0x5E8; // C_CommandContext
        pub const m_nInButtonsWhichAreToggles: usize = 0x6A0; // uint64
        pub const m_nTickBase: usize = 0x6A8; // uint32
        pub const m_hPawn: usize = 0x6AC; // CHandle<C_BasePlayerPawn>
        pub const m_bKnownTeamMismatch: usize = 0x6B0; // bool
        pub const m_hPredictedPawn: usize = 0x6B4; // CHandle<C_BasePlayerPawn>
        pub const m_nSplitScreenSlot: usize = 0x6B8; // CSplitScreenSlot
        pub const m_hSplitOwner: usize = 0x6BC; // CHandle<CBasePlayerController>
        pub const m_hSplitScreenPlayers: usize = 0x6C0; // CUtlVector<CHandle<CBasePlayerController>>
        pub const m_bIsHLTV: usize = 0x6D8; // bool
        pub const m_iConnected: usize = 0x6DC; // PlayerConnectedState
        pub const m_iszPlayerName: usize = 0x6E0; // char[128]
        pub const m_steamID: usize = 0x768; // uint64
        pub const m_bIsLocalPlayerController: usize = 0x770; // bool
        pub const m_bNoClipEnabled: usize = 0x771; // bool
        pub const m_iDesiredFOV: usize = 0x774; // uint32
    }

    pub mod C_BaseEntity {
        pub const m_CBodyComponent: usize = 0x38; // CBodyComponent*
        pub const m_NetworkTransmitComponent: usize = 0x40; // CNetworkTransmitComponent
        pub const m_nLastThinkTick: usize = 0x328; // GameTick_t
        pub const m_pGameSceneNode: usize = 0x330; // CGameSceneNode*
        pub const m_pRenderComponent: usize = 0x338; // CRenderComponent*
        pub const m_pCollision: usize = 0x340; // CCollisionProperty*
        pub const m_pModifierProp: usize = 0x348; // CModifierProperty*
        pub const m_iMaxHealth: usize = 0x350; // int32
        pub const m_iHealth: usize = 0x354; // int32
        pub const m_lifeState: usize = 0x358; // uint8
        pub const m_bTakesDamage: usize = 0x359; // bool
        pub const m_nTakeDamageFlags: usize = 0x360; // TakeDamageFlags_t
        pub const m_nPlatformType: usize = 0x368; // EntityPlatformTypes_t
        pub const m_ubInterpolationFrame: usize = 0x369; // uint8
        pub const m_hSceneObjectController: usize = 0x36C; // CHandle<C_BaseEntity>
        pub const m_nNoInterpolationTick: usize = 0x370; // int32
        pub const m_nVisibilityNoInterpolationTick: usize = 0x374; // int32
        pub const m_flProxyRandomValue: usize = 0x378; // float32
        pub const m_iEFlags: usize = 0x37C; // int32
        pub const m_nWaterType: usize = 0x380; // uint8
        pub const m_bInterpolateEvenWithNoModel: usize = 0x381; // bool
        pub const m_bPredictionEligible: usize = 0x382; // bool
        pub const m_bApplyLayerMatchIDToModel: usize = 0x383; // bool
        pub const m_tokLayerMatchID: usize = 0x384; // CUtlStringToken
        pub const m_nSubclassID: usize = 0x388; // CUtlStringToken
        pub const m_nSimulationTick: usize = 0x398; // int32
        pub const m_iCurrentThinkContext: usize = 0x39C; // int32
        pub const m_aThinkFunctions: usize = 0x3A0; // CUtlVector<thinkfunc_t>
        pub const m_bDisabledContextThinks: usize = 0x3B8; // bool
        pub const m_flAnimTime: usize = 0x3BC; // float32
        pub const m_flSimulationTime: usize = 0x3C0; // float32
        pub const m_nSceneObjectOverrideFlags: usize = 0x3C4; // uint8
        pub const m_bHasSuccessfullyInterpolated: usize = 0x3C5; // bool
        pub const m_bHasAddedVarsToInterpolation: usize = 0x3C6; // bool
        pub const m_bRenderEvenWhenNotSuccessfullyInterpolated: usize = 0x3C7; // bool
        pub const m_nInterpolationLatchDirtyFlags: usize = 0x3C8; // int32[2]
        pub const m_ListEntry: usize = 0x3D0; // uint16[11]
        pub const m_flCreateTime: usize = 0x3E8; // GameTime_t
        pub const m_flSpeed: usize = 0x3EC; // float32
        pub const m_EntClientFlags: usize = 0x3F0; // uint16
        pub const m_bClientSideRagdoll: usize = 0x3F2; // bool
        pub const m_iTeamNum: usize = 0x3F3; // uint8
        pub const m_spawnflags: usize = 0x3F4; // uint32
        pub const m_nNextThinkTick: usize = 0x3F8; // GameTick_t
        pub const m_fFlags: usize = 0x400; // uint32
        pub const m_vecAbsVelocity: usize = 0x404; // Vector
        pub const m_vecServerVelocity: usize = 0x410; // CNetworkVelocityVector
        pub const m_vecVelocity: usize = 0x438; // CNetworkVelocityVector
        pub const m_hEffectEntity: usize = 0x518; // CHandle<C_BaseEntity>
        pub const m_hOwnerEntity: usize = 0x51C; // CHandle<C_BaseEntity>
        pub const m_MoveCollide: usize = 0x520; // MoveCollide_t
        pub const m_MoveType: usize = 0x521; // MoveType_t
        pub const m_nActualMoveType: usize = 0x522; // MoveType_t
        pub const m_flWaterLevel: usize = 0x524; // float32
        pub const m_fEffects: usize = 0x528; // uint32
        pub const m_hGroundEntity: usize = 0x52C; // CHandle<C_BaseEntity>
        pub const m_nGroundBodyIndex: usize = 0x530; // int32
        pub const m_flFriction: usize = 0x534; // float32
        pub const m_flElasticity: usize = 0x538; // float32
        pub const m_flGravityScale: usize = 0x53C; // float32
        pub const m_flTimeScale: usize = 0x540; // float32
        pub const m_bAnimatedEveryTick: usize = 0x544; // bool
        pub const m_flNavIgnoreUntilTime: usize = 0x548; // GameTime_t
        pub const m_hThink: usize = 0x54C; // uint16
        pub const m_fBBoxVisFlags: usize = 0x558; // uint8
        pub const m_bPredictable: usize = 0x559; // bool
        pub const m_bRenderWithViewModels: usize = 0x55A; // bool
        pub const m_nFirstPredictableCommand: usize = 0x55C; // int32
        pub const m_nLastPredictableCommand: usize = 0x560; // int32
        pub const m_hOldMoveParent: usize = 0x564; // CHandle<C_BaseEntity>
        pub const m_Particles: usize = 0x568; // CParticleProperty
        pub const m_vecAngVelocity: usize = 0x598; // QAngle
        pub const m_DataChangeEventRef: usize = 0x5A4; // int32
        pub const m_dependencies: usize = 0x5A8; // CUtlVector<CEntityHandle>
        pub const m_nCreationTick: usize = 0x5C0; // int32
        pub const m_bAnimTimeChanged: usize = 0x5CD; // bool
        pub const m_bSimulationTimeChanged: usize = 0x5CE; // bool
        pub const m_sUniqueHammerID: usize = 0x5D8; // CUtlString
    }

    pub mod C_BasePlayerPawn {
        pub const m_pWeaponServices: usize = 0xF48; // CPlayer_WeaponServices*
        pub const m_pItemServices: usize = 0xF50; // CPlayer_ItemServices*
        pub const m_pAutoaimServices: usize = 0xF58; // CPlayer_AutoaimServices*
        pub const m_pObserverServices: usize = 0xF60; // CPlayer_ObserverServices*
        pub const m_pWaterServices: usize = 0xF68; // CPlayer_WaterServices*
        pub const m_pUseServices: usize = 0xF70; // CPlayer_UseServices*
        pub const m_pFlashlightServices: usize = 0xF78; // CPlayer_FlashlightServices*
        pub const m_pCameraServices: usize = 0xF80; // CPlayer_CameraServices*
        pub const m_pMovementServices: usize = 0xF88; // CPlayer_MovementServices*
        pub const m_ServerViewAngleChanges: usize = 0xF98; // C_UtlVectorEmbeddedNetworkVar<ViewAngleServerChange_t>
        pub const v_angle: usize = 0x1000; // QAngle
        pub const v_anglePrevious: usize = 0x100C; // QAngle
        pub const m_iHideHUD: usize = 0x1018; // uint32
        pub const m_skybox3d: usize = 0x1020; // sky3dparams_t
        pub const m_flDeathTime: usize = 0x10B0; // GameTime_t
        pub const m_vecPredictionError: usize = 0x10B4; // Vector
        pub const m_flPredictionErrorTime: usize = 0x10C0; // GameTime_t
        pub const m_vecLastCameraSetupLocalOrigin: usize = 0x10E0; // Vector
        pub const m_flLastCameraSetupTime: usize = 0x10EC; // GameTime_t
        pub const m_flFOVSensitivityAdjust: usize = 0x10F0; // float32
        pub const m_flMouseSensitivity: usize = 0x10F4; // float32
        pub const m_vOldOrigin: usize = 0x10F8; // Vector
        pub const m_flOldSimulationTime: usize = 0x1104; // float32
        pub const m_nLastExecutedCommandNumber: usize = 0x1108; // int32
        pub const m_nLastExecutedCommandTick: usize = 0x110C; // int32
        pub const m_hController: usize = 0x1110; // CHandle<CBasePlayerController>
        pub const m_hDefaultController: usize = 0x1114; // CHandle<CBasePlayerController>
        pub const m_bIsSwappingToPredictableController: usize = 0x1118; // bool
    }

    pub mod C_GameRules {
        pub const __m_pChainEntity: usize = 0x8; // CNetworkVarChainer
        pub const m_nTotalPausedTicks: usize = 0x30; // int32
        pub const m_nPauseStartTick: usize = 0x34; // int32
        pub const m_bGamePaused: usize = 0x38; // bool
    }

    pub mod CGameSceneNode {
        pub const m_nodeToWorld: usize = 0x10; // CTransform
        pub const m_pOwner: usize = 0x30; // CEntityInstance*
        pub const m_pParent: usize = 0x38; // CGameSceneNode*
        pub const m_pChild: usize = 0x40; // CGameSceneNode*
        pub const m_pNextSibling: usize = 0x48; // CGameSceneNode*
        pub const m_hParent: usize = 0x78; // CGameSceneNodeHandle
        pub const m_vecOrigin: usize = 0x88; // CNetworkOriginCellCoordQuantizedVector
        pub const m_angRotation: usize = 0xC0; // QAngle
        pub const m_flScale: usize = 0xCC; // float32
        pub const m_vecAbsOrigin: usize = 0xD0; // Vector
        pub const m_angAbsRotation: usize = 0xDC; // QAngle
        pub const m_flAbsScale: usize = 0xE8; // float32
        pub const m_nParentAttachmentOrBone: usize = 0xEC; // int16
        pub const m_bDebugAbsOriginChanges: usize = 0xEE; // bool
        pub const m_bDormant: usize = 0xEF; // bool
        pub const m_bForceParentToBeNetworked: usize = 0xF0; // bool
        pub const m_bDirtyHierarchy: usize = 0x0; // bitfield:1
        pub const m_bDirtyBoneMergeInfo: usize = 0x0; // bitfield:1
        pub const m_bNetworkedPositionChanged: usize = 0x0; // bitfield:1
        pub const m_bNetworkedAnglesChanged: usize = 0x0; // bitfield:1
        pub const m_bNetworkedScaleChanged: usize = 0x0; // bitfield:1
        pub const m_bWillBeCallingPostDataUpdate: usize = 0x0; // bitfield:1
        pub const m_bBoneMergeFlex: usize = 0x0; // bitfield:1
        pub const m_nLatchAbsOrigin: usize = 0x0; // bitfield:2
        pub const m_bDirtyBoneMergeBoneToRoot: usize = 0x0; // bitfield:1
        pub const m_nHierarchicalDepth: usize = 0xF3; // uint8
        pub const m_nHierarchyType: usize = 0xF4; // uint8
        pub const m_nDoNotSetAnimTimeInInvalidatePhysicsCount: usize = 0xF5; // uint8
        pub const m_name: usize = 0xF8; // CUtlStringToken
        pub const m_hierarchyAttachName: usize = 0x138; // CUtlStringToken
        pub const m_flZOffset: usize = 0x13C; // float32
        pub const m_flClientLocalScale: usize = 0x140; // float32
        pub const m_vRenderOrigin: usize = 0x144; // Vector
    }

    pub mod CEntityIdentity {
        pub const m_nameStringableIndex: usize = 0x14; // int32
        pub const m_name: usize = 0x18; // CUtlSymbolLarge
        pub const m_designerName: usize = 0x20; // CUtlSymbolLarge
        pub const m_flags: usize = 0x30; // uint32
        pub const m_worldGroupId: usize = 0x38; // WorldGroupId_t
        pub const m_fDataObjectTypes: usize = 0x3C; // uint32
        pub const m_PathIndex: usize = 0x40; // ChangeAccessorFieldPathIndex_t
        pub const m_pPrev: usize = 0x58; // CEntityIdentity*
        pub const m_pNext: usize = 0x60; // CEntityIdentity*
        pub const m_pPrevByClass: usize = 0x68; // CEntityIdentity*
        pub const m_pNextByClass: usize = 0x70; // CEntityIdentity*
    }

    pub mod CCitadelAbilityComponent {
        pub const m_vecAbilities: usize = 0x70; // C_NetworkUtlVectorBase<CHandle<C_CitadelBaseAbility>>
        pub const m_arPendingAsyncAbilityReservationSlots: usize = 0x88; // C_NetworkUtlVectorBase<int32>
        pub const m_arPendingAsyncAbilityReservationAbilityIDs: usize = 0xA0; // C_NetworkUtlVectorBase<int32>
        pub const m_hSelectedAbility: usize = 0xB8; // CHandle<C_CitadelBaseAbility>
        pub const m_hPreviouslySelectedAbility: usize = 0xBC; // CHandle<C_BaseEntity>
        pub const m_bPreviousAbilityQueued: usize = 0xC0; // bool
        pub const m_flTimeScale: usize = 0xC4; // float32
        pub const m_flParticleTimeScale: usize = 0xC8; // float32
        pub const m_bInInterruptState: usize = 0xCC; // bool
        pub const m_ResourceStamina: usize = 0xD0; // AbilityResource_t
        pub const m_ResourceAbility: usize = 0xF0; // AbilityResource_t
        pub const m_vecConsumedComponents: usize = 0x110; // C_UtlVectorEmbeddedNetworkVar<ConsumedComponentState_t>
        pub const m_nExecuteAbilityMask: usize = 0x1C0; // uint32
        pub const m_bSelectedEffectsStarted: usize = 0x1C8; // bool
    }

    pub mod C_CitadelPlayerPawn {
        pub const m_angEyeAngles: usize = 0x11B8; // QAngle
        pub const m_flLastCombatTime: usize = 0x1248; // GameTime_t
        pub const m_angClientCamera: usize = 0x124C; // QAngle
        pub const m_eZipLineLaneColor: usize = 0x1258; // CMsgLaneColor
        pub const m_nLevel: usize = 0x125C; // int32
        pub const m_nCurrencies: usize = 0x1260; // int32[4]
        pub const m_nSpentCurrencies: usize = 0x1270; // int32[4]
        pub const m_flLastSpawnTime: usize = 0x1280; // GameTime_t
        pub const m_flRespawnTime: usize = 0x1284; // GameTime_t
        pub const m_bInRegenerationZone: usize = 0x1288; // bool
        pub const m_bInItemShopZone: usize = 0x1289; // bool
        pub const m_bInHideoutZone: usize = 0x128A; // bool
        pub const m_timeRevealedOnMinimapByNPC: usize = 0x128C; // GameTime_t
        pub const m_vecFullSellPriceItems: usize = 0x1290; // C_NetworkUtlVectorBase<CUtlStringToken>
        pub const m_vecFullSellPriceAbilityUpgrades: usize = 0x12A8; // C_NetworkUtlVectorBase<FullSellPriceAbilityUpgrades_t>
        pub const m_bNetworkDisconnected: usize = 0x12C0; // bool
        pub const m_bHasIncomingThreats: usize = 0x12C1; // bool
        pub const m_bLearningAbility: usize = 0x12C2; // bool
        pub const m_nFlashStartTick: usize = 0x12C4; // int32
        pub const m_nFlashMaxStartTick: usize = 0x12C8; // int32
        pub const m_nFlashFadeStartTick: usize = 0x12CC; // int32
        pub const m_nFlashEndTick: usize = 0x12D0; // int32
        pub const m_nFlashMaxAlpha: usize = 0x12D4; // int8
        pub const m_nDeducedLane: usize = 0x12D8; // int32
        pub const m_nSuccessiveDucks: usize = 0x12DC; // int8
        pub const m_flLastDuckTime: usize = 0x12E0; // GameTime_t
        pub const m_vWallJumpFacingDir: usize = 0x12E4; // Vector
        pub const m_eWallJumpFacing: usize = 0x12F0; // EWallJumpFacing
        pub const m_vCurrentWallNormal: usize = 0x12F4; // Vector
        pub const m_vLastWallCollidedWithNormal: usize = 0x1300; // Vector
        pub const m_vLastValidWallJumpNormal: usize = 0x130C; // Vector
        pub const m_vLastValidWallJumpNormal_PlayerPosition: usize = 0x1318; // Vector
        pub const m_flLastWallJumpTime: usize = 0x1324; // GameTime_t
        pub const m_bDismissedReportCard: usize = 0x1328; // bool
        pub const m_flCurrentHealingAmount: usize = 0x132C; // float32
        pub const m_angLockedEyeAngles: usize = 0x1330; // QAngle
        pub const m_CCitadelAbilityComponent: usize = 0x1340; // CCitadelAbilityComponent
        pub const m_CCitadelHeroComponent: usize = 0x1510; // CCitadelHeroComponent
        pub const m_flRichPresenceUpdateInterval: usize = 0x15DC; // float32
        pub const m_bAnimGraphMovementClipped: usize = 0x16D8; // bool
        pub const m_bAnimGraphMovementDisableGravity: usize = 0x16D9; // bool
        pub const m_bAnimGraphMovementDirectAirControl: usize = 0x16DA; // bool
        pub const m_flPredTimeSlowedStart: usize = 0x16DC; // GameTime_t
        pub const m_flPredTimeSlowedEnd: usize = 0x16E0; // GameTime_t
        pub const m_flPredSlowSpeed: usize = 0x16E4; // float32
        pub const m_flTimeSlowedStart: usize = 0x16E8; // GameTime_t[4]
        pub const m_flTimeSlowedEnd: usize = 0x16F8; // GameTime_t[4]
        pub const m_flSlowSpeed: usize = 0x1708; // float32[4]
        pub const m_flSprintAnimSuppressEndTime: usize = 0x1718; // GameTime_t
        pub const m_iCurSlowSlot: usize = 0x171C; // int32
        pub const m_bLocoLeanTriggeredForDirection: usize = 0x1720; // bool
        pub const m_bLocoRunToStopCanTrigger: usize = 0x1721; // bool
        pub const m_flCrouchFraction: usize = 0x1724; // float32
        pub const m_flCrouchSpeed: usize = 0x1728; // float32
        pub const m_vShootTestOffsetStanding: usize = 0x172C; // Vector
        pub const m_vShootTestOffsetCrouching: usize = 0x1738; // Vector
        pub const m_leanStartTime: usize = 0x1744; // GameTime_t
    }

    pub mod C_CitadelBaseAbility {
        pub const m_vecIntrinsicModifiers: usize = 0x6B0; // CUtlVector<CModifierHandleTyped<CCitadelModifier>>
        pub const m_pCastDelayAutoModifier: usize = 0x6C8; // CModifierHandleTyped<CCitadelModifier>
        pub const m_pChannelAutoModifier: usize = 0x6E0; // CModifierHandleTyped<CCitadelModifier>
        pub const m_strUsedCastGraphParam: usize = 0x6F8; // CGlobalSymbol
        pub const m_nCastParamNeedsResetTick: usize = 0x700; // int32
        pub const m_bIsCoolingDownInternal: usize = 0x704; // bool
        pub const m_flCancelMashProtectionEndTime: usize = 0x708; // GameTime_t
        pub const m_flCancelLockoutEndTime: usize = 0x70C; // GameTime_t
        pub const m_bChanneling: usize = 0x728; // bool
        pub const m_bInCastDelay: usize = 0x729; // bool
        pub const m_vecImbuedByAbilitiyIDs: usize = 0x730; // C_NetworkUtlVectorBase<CUtlStringToken>
        pub const m_bCanBeUpgraded: usize = 0x748; // bool
        pub const m_eStolenInSlot: usize = 0x750; // CitadelStolenAbilitySlot_t
        pub const m_nUpgradeBits: usize = 0x760; // int32
        pub const m_iBucketID: usize = 0x764; // EAbilityBucketType
        pub const m_bToggleState: usize = 0x768; // bool
        pub const m_flCooldownStart: usize = 0x76C; // GameTime_t
        pub const m_flCooldownEnd: usize = 0x770; // GameTime_t
        pub const m_flCastCompletedTime: usize = 0x774; // GameTime_t
        pub const m_flChannelStartTime: usize = 0x778; // GameTime_t
        pub const m_flCastDelayStartTime: usize = 0x77C; // GameTime_t
        pub const m_eAbilitySlot: usize = 0x780; // EAbilitySlots_t
        pub const m_flPostCastDelayEndTime: usize = 0x784; // GameTime_t
        pub const m_iRemainingCharges: usize = 0x788; // int32
        pub const m_flChargeRechargeStart: usize = 0x78C; // GameTime_t
        pub const m_flChargeRechargeEnd: usize = 0x790; // GameTime_t
        pub const m_flMovementControlActiveTime: usize = 0x794; // GameTime_t
        pub const m_flSelectedChangedTime: usize = 0x798; // GameTime_t
        pub const m_flAltCastHoldStartTime: usize = 0x79C; // GameTime_t
        pub const m_flAltCastDoubleTapStartTime: usize = 0x7A0; // GameTime_t
        pub const m_bCanBeImbued: usize = 0x7A4; // bool
        pub const m_nImbuedAbilityID: usize = 0x7A8; // CUtlStringToken
        pub const m_bSelectionModeIsAltMode: usize = 0x7AC; // bool
    }

    pub mod PlayerDataGlobal_t {
        pub const m_iLevel: usize = 0x8; // int32
        pub const m_iMaxAmmo: usize = 0xC; // int32
        pub const m_iHealthMax: usize = 0x10; // int32
        pub const m_flHealthRegen: usize = 0x14; // float32
        pub const m_flRespawnTime: usize = 0x18; // GameTime_t
        pub const m_nHeroID: usize = 0x1C; // HeroID_t
        pub const m_iGoldNetWorth: usize = 0x20; // int32
        pub const m_iAPNetWorth: usize = 0x24; // int32
        pub const m_iCreepGold: usize = 0x28; // int32
        pub const m_iCreepGoldSoloBonus: usize = 0x2C; // int32
        pub const m_iCreepGoldKill: usize = 0x30; // int32
        pub const m_iCreepGoldAirOrb: usize = 0x34; // int32
        pub const m_iCreepGoldGroundOrb: usize = 0x38; // int32
        pub const m_iCreepGoldDeny: usize = 0x3C; // int32
        pub const m_iCreepGoldNeutral: usize = 0x40; // int32
        pub const m_iFarmBaseline: usize = 0x44; // int32
        pub const m_iHealth: usize = 0x48; // int32
        pub const m_iPlayerKills: usize = 0x4C; // int32
        pub const m_iPlayerAssists: usize = 0x50; // int32
        pub const m_iDeaths: usize = 0x54; // int32
        pub const m_iDenies: usize = 0x58; // int32
        pub const m_iLastHits: usize = 0x5C; // int32
        pub const m_bAlive: usize = 0x60; // bool
        pub const m_nHeroDraftPosition: usize = 0x64; // int32
        pub const m_bUltimateTrained: usize = 0x68; // bool
        pub const m_flUltimateCooldownStart: usize = 0x6C; // GameTime_t
        pub const m_flUltimateCooldownEnd: usize = 0x70; // GameTime_t
        pub const m_bHasRejuvenator: usize = 0x74; // bool
        pub const m_bHasRebirth: usize = 0x75; // bool
        pub const m_bFlaggedAsCheater: usize = 0x76; // bool
        pub const m_iHeroDamage: usize = 0x78; // int32
        pub const m_iHeroHealing: usize = 0x7C; // int32
        pub const m_iSelfHealing: usize = 0x80; // int32
        pub const m_iObjectiveDamage: usize = 0x84; // int32
        pub const m_nHeroAbilityUpgradeBits: usize = 0x88; // int32[4]
        pub const m_vecUpgrades: usize = 0x98; // C_NetworkUtlVectorBase<CUtlStringToken>
        pub const m_vecBonusCounterAbilities: usize = 0xB0; // C_NetworkUtlVectorBase<CUtlStringToken>
        pub const m_vecBonusCounterValues: usize = 0xC8; // C_NetworkUtlVectorBase<int32>
        pub const m_vecBonusCounterModifiers: usize = 0xE0; // C_NetworkUtlVectorBase<CUtlStringToken>
        pub const m_vecModifierBonusCounterValues: usize = 0xF8; // C_NetworkUtlVectorBase<int32>
        pub const m_tHeldItem: usize = 0x110; // CUtlStringToken
        pub const m_vecImbuements: usize = 0x118; // C_UtlVectorEmbeddedNetworkVar<ItemImbuementPair_t>
        pub const m_vecDynamicAbilityValues: usize = 0x180; // C_UtlVectorEmbeddedNetworkVar<DynamicAbilityValues_t>
        pub const m_vecStatViewerModifierValues: usize = 0x1E8; // C_UtlVectorEmbeddedNetworkVar<StatViewerModifierValues_t>
        pub const m_vecStolenAbilities: usize = 0x250; // C_UtlVectorEmbeddedNetworkVar<StolenAbilityPair_t>
    }

    pub mod CCitadelPlayerController {
        pub const m_ePlayState: usize = 0x7E4; // EPlayerPlayState
        pub const m_iGuidedBotMatchLastHits: usize = 0x7E8; // int32
        pub const m_iGuidedBotMatchOrbsSecured: usize = 0x7EC; // int32
        pub const m_iGuidedBotMatchOrbsDenied: usize = 0x7F0; // int32
        pub const m_iGuidedBotMatchDamageToGuardians: usize = 0x7F4; // int32
        pub const m_iGuidedBotMatchDamageToPlayers: usize = 0x7F8; // int32
        pub const m_iGuidedBotMatchDamageTaken: usize = 0x7FC; // int32
        pub const m_iGuidedBotMatchNetWorth: usize = 0x800; // int32
        pub const m_iGuidedBotMatchModsPurchased: usize = 0x804; // int32
        pub const m_iGuidedBotMatchAbilityUpgrades: usize = 0x808; // int32
        pub const m_flGuideBotMatchLastTaskNagVO: usize = 0x80C; // float32
        pub const m_flGuideBotLastTimeTaskCompleted: usize = 0x810; // float32
        pub const m_eGuidedBotMatchObjective: usize = 0x814; // EGuidedBotMatchObjective
        pub const m_nCurrentRank: usize = 0x818; // int32
        pub const m_nAssignedLane: usize = 0x81C; // int8
        pub const m_nOriginalLaneAssignment: usize = 0x81D; // int8
        pub const m_bIsKingPanda: usize = 0x81E; // bool
        pub const m_bBotDisconnectTakeover: usize = 0x81F; // bool
        pub const m_bInTeamChat: usize = 0x820; // bool
        pub const m_bInPartyChat: usize = 0x821; // bool
        pub const m_unHeroBuildID: usize = 0x824; // HeroBuildID_t
        pub const m_bLaneSwapLocked: usize = 0x828; // bool
        pub const m_vecLaneSwapRequests: usize = 0x830; // C_NetworkUtlVectorBase<CHandle<C_BaseEntity>>
        pub const m_vecLaneSwapRejects: usize = 0x848; // C_NetworkUtlVectorBase<CHandle<C_BaseEntity>>
        pub const m_vecMutedPlayers: usize = 0x860; // C_NetworkUtlVectorBase<int32>
        pub const m_hHeroPawn: usize = 0x878; // CHandle<C_CitadelPlayerPawn>
        pub const m_PlayerDataGlobal: usize = 0x8B8; // PlayerDataGlobal_t
        pub const m_nDeathReplayAvailable: usize = 0xB70; // int8
        pub const m_unLobbyPlayerSlot: usize = 0xB71; // CitadelLobbyPlayerSlot_t
        pub const m_bHasCheckedFriendName: usize = 0xB72; // bool
        pub const m_sFriendName: usize = 0xB78; // CUtlString
    }

    pub mod CSkeletonInstance {
        pub const m_modelState: usize = 0x170; // CModelState
        pub const m_bIsAnimationEnabled: usize = 0x480; // bool
        pub const m_bUseParentRenderBounds: usize = 0x481; // bool
        pub const m_bDisableSolidCollisionsForHierarchy: usize = 0x482; // bool
        pub const m_bDirtyMotionType: usize = 0x0; // bitfield:1
        pub const m_bIsGeneratingLatchedParentSpaceState: usize = 0x0; // bitfield:1
        pub const m_materialGroup: usize = 0x484; // CUtlStringToken
        pub const m_nHitboxSet: usize = 0x488; // uint8
    }

    pub mod CPlayer_ObserverServices {
        pub const m_iObserverMode: usize = 0x40; // uint8
        pub const m_hObserverTarget: usize = 0x44; // CHandle<C_BaseEntity>
        pub const m_iObserverLastMode: usize = 0x48; // ObserverMode_t
        pub const m_bForcedObserverMode: usize = 0x4C; // bool
        pub const m_flObserverChaseDistance: usize = 0x50; // float32
        pub const m_flObserverChaseDistanceCalcTime: usize = 0x54; // GameTime_t
    }

    pub mod CPlayer_CameraServices {
        pub const m_vecPunchAngle: usize = 0x40; // QAngle
        pub const m_vecPunchAngleVel: usize = 0xD0; // QAngle
        pub const m_nPunchAngleJoltTickClientSide: usize = 0x160; // GameTick_t
        pub const m_nPunchAngleJoltTick: usize = 0x164; // GameTick_t
        pub const m_PlayerFog: usize = 0x168; // C_fogplayerparams_t
        pub const m_hColorCorrectionCtrl: usize = 0x1A8; // CHandle<C_ColorCorrection>
        pub const m_hViewEntity: usize = 0x1AC; // CHandle<C_BaseEntity>
        pub const m_hTonemapController: usize = 0x1B0; // CHandle<C_TonemapController2>
        pub const m_audio: usize = 0x1B8; // audioparams_t
        pub const m_PostProcessingVolumes: usize = 0x230; // C_NetworkUtlVectorBase<CHandle<C_PostProcessingVolume>>
        pub const m_flOldPlayerZ: usize = 0x248; // float32
        pub const m_flOldPlayerViewOffsetZ: usize = 0x24C; // float32
        pub const m_CurrentFog: usize = 0x250; // fogparams_t
        pub const m_hOldFogController: usize = 0x2B8; // CHandle<C_FogController>
        pub const m_bOverrideFogColor: usize = 0x2BC; // bool[5]
        pub const m_OverrideFogColor: usize = 0x2C1; // Color[5]
        pub const m_bOverrideFogStartEnd: usize = 0x2D5; // bool[5]
        pub const m_fOverrideFogStart: usize = 0x2DC; // float32[5]
        pub const m_fOverrideFogEnd: usize = 0x2F0; // float32[5]
        pub const m_hActivePostProcessingVolume: usize = 0x304; // CHandle<C_PostProcessingVolume>
        pub const m_angDemoViewAngles: usize = 0x308; // QAngle
    }

    pub mod CCitadel_Ability_PrimaryWeapon {
        pub const m_flNextPrimaryAttack: usize = 0xDC0; // GameTime_t
        pub const m_flNextPrimaryAttackStartTime: usize = 0xDC4; // GameTime_t
        pub const m_iClip: usize = 0xDC8; // int32
        pub const m_iBonusClip: usize = 0xDCC; // int32
        pub const m_nNumContinuousShots: usize = 0xDD0; // int32
        pub const m_flContinuousShotStartTime: usize = 0xDD4; // GameTime_t
        pub const m_flSpreadPenalty: usize = 0xDD8; // float32
        pub const m_flZoomTime: usize = 0xDDC; // GameTime_t
        pub const m_flZoomOutTime: usize = 0xDE0; // GameTime_t
        pub const m_iSpreadIndex: usize = 0xDE4; // int8
        pub const m_nShotRecoilIndex: usize = 0xDE6; // int16
        pub const m_flNextShotRecoilRecoveryTime: usize = 0xDE8; // GameTime_t
        pub const m_bIsZoomed: usize = 0xDEC; // bool
        pub const m_nBurstShotsRemaining: usize = 0xDED; // uint8
        pub const m_nShotNumber: usize = 0xDF0; // uint32
        pub const m_bInReload: usize = 0xDF4; // bool
        pub const m_bSingleShotReloadFirstBullet: usize = 0xDF5; // bool
        pub const m_reloadQueuedStartTime: usize = 0xDF8; // GameTime_t
        pub const m_flReloadAvailableTime: usize = 0xDFC; // GameTime_t
        pub const m_bCanActiveReload: usize = 0xE00; // bool
        pub const m_flLastAttackTime: usize = 0xE04; // GameTime_t
        pub const m_flNextAttackDelayStartTime: usize = 0xE08; // GameTime_t
        pub const m_flNextAttackDelayEndTime: usize = 0xE0C; // GameTime_t
        pub const m_flAttackDelayPauseTotalTime: usize = 0xE10; // float32
        pub const m_flAttackDelayPauseEndTime: usize = 0xE14; // GameTime_t
        pub const m_eNextAttackDelayReason: usize = 0xE18; // ENextAttackDelayReason_t
        pub const m_bInputPressedWhileSelected: usize = 0xE1C; // bool
        pub const m_eActiveFireMode: usize = 0xE20; // EFireMode_t
        pub const m_angRecoilAngles: usize = 0xE24; // QAngle
        pub const m_angRecoilToAdd: usize = 0xE30; // QAngle
        pub const m_angRecoilRecovery: usize = 0xE3C; // QAngle
        pub const m_flRecoilStartTime: usize = 0xE48; // GameTime_t
        pub const m_flRecoilRecoverySpeed: usize = 0xE4C; // float32
        pub const m_flAddApproachSpeed: usize = 0xE50; // float32
        pub const m_bFireBackwards: usize = 0xE54; // bool
        pub const m_currentSpread: usize = 0xE58; // float32
        pub const m_currentMaxSpread: usize = 0xE5C; // float32
        pub const m_currentFireSpread: usize = 0xE60; // float32
        pub const m_flCurrentSpinRate: usize = 0xE64; // float32
        pub const m_fFireDuration: usize = 0xE6C; // float32
        pub const m_bFireOnEmpty: usize = 0xE71; // bool
        pub const m_bHasReleasedForSemiAuto: usize = 0xE72; // bool
        pub const m_flNextDisarmSound: usize = 0xE74; // GameTime_t
    }

    pub mod CCitadel_Ability_Slide {
        pub const m_flGroundDashSlideTime: usize = 0xE18; // CCitadelAutoScaledTime
        pub const m_flSlowGetupStartTime: usize = 0xE30; // GameTime_t
        pub const m_bShouldTriggerSlowGetup: usize = 0xE34; // bool
        pub const m_bWantsSlide: usize = 0xE35; // bool
        pub const m_bAirborneWhenDuckPressed: usize = 0xE36; // bool
        pub const m_bIsSliding: usize = 0xE37; // bool
        pub const m_bSlideIsSticky: usize = 0xE38; // bool
        pub const m_flSpeedAdjust: usize = 0xE3C; // float32
        pub const m_flDuckPressedTime: usize = 0xE40; // GameTime_t
        pub const m_flSlideChangeTime: usize = 0xE44; // GameTime_t
        pub const m_flSlidingOnFlatStartTime: usize = 0xE48; // GameTime_t
        pub const m_nJumpsThisSlideSession: usize = 0xE4C; // int32
        pub const m_flOnGroundStartTime: usize = 0xE50; // GameTime_t
        pub const m_flDashSlideStartTime: usize = 0xE54; // GameTime_t
        pub const m_bStartedSlideViaProbeSlope: usize = 0xE58; // bool
        pub const m_nSlideEffectIndex: usize = 0xE5C; // ParticleIndex_t
    }
}
