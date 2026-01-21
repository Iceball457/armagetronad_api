use std::fmt::Display;

pub enum Command {
    AccessLevel,
    AccessLevelAnnounceLogin,
    AccessLevelAutokickImmunity,
    AccessLevelChat,
    AccessLevelChatTimeout,
    AccessLevelHideOf,
    AccessLevelHideTo,
    AccessLevelIps,
    AccessLevelListAdmins,
    AccessLevelListAdminsSeeEveryone,
    AccessLevelNver,
    AccessLevelPlay,
    AccessLevelPlaySliders,
    AccessLevelPlaySliding,
    AccessLevelRtfm,
    AccessLevelShout,
    AccessLevelShuffleUp,
    AccessLevelSpyMsg,
    AccessLevelSpyTeam,
    AccessLevelVoteCommand,
    AccessLevelVoteCommandExecute,
    AccessLevelVoteInclude,
    AccessLevelVoteIncludeExecute,
    AccessLevelVoteKick,
    AccessLevelVoteSilence,
    AccessLevelVoteSuspend,
    AddHelpTopic,
    AddMasterServer,
    Admins,
    AdminListColorsBestBlue,
    AdminListColorsBestGreen,
    AdminListColorsBestRed,
    AdminListColorsWorstBlue,
    AdminListColorsWorstGreen,
    AdminListColorsWorstRed,
    AdminListMinAccessLevel,
    AiIq,
    AliveLocx,
    AliveLocy,
    AliveSize,
    AllowCam1_0,
    AllowCam1_1,
    AllowCam1_2,
    AllowCam1_3,
    AllowCam1_4,
    AllowCam1_5,
    AllowCam1_6,
    AllowCam2_0,
    AllowCam2_1,
    AllowCam2_2,
    AllowCam2_3,
    AllowCam2_4,
    AllowCam2_5,
    AllowCam2_6,
    AllowCam3_0,
    AllowCam3_1,
    AllowCam3_2,
    AllowCam3_3,
    AllowCam3_4,
    AllowCam3_5,
    AllowCam3_6,
    AllowCam4_0,
    AllowCam4_1,
    AllowCam4_2,
    AllowCam4_3,
    AllowCam4_4,
    AllowCam4_5,
    AllowCam4_6,
    AllowControlDuringChat,
    AllowEnemiesSameClient,
    AllowEnemiesSameIp,
    AllowImposters,
    AllowImpostors,
    AllowRenamePlayer,
    AllowTeamChange,
    AllowTeamChangePlayer,
    AllowTeamNameColor,
    AllowTeamNamePlayer,
    AllowVoting,
    AllowVotingSpectator,
    AlphaBlend,
    AntiSpoof,
    ArenaAxes,
    ArenaAxesOverride,
    ArmagetronLastScreenmode,
    ArmagetronLastWindowsize,
    ArmagetronScreenmode,
    ArmagetronVsync,
    ArmagetronVsyncLast,
    ArmagetronWindowsize,
    AuthorityBlacklist,
    AuthorityLevel,
    AuthorityWhitelist,
    AutoAis,
    AutoIncam1,
    AutoIncam2,
    AutoIncam3,
    AutoIncam4,
    AutoIq,
    AutoLogin1,
    AutoLogin2,
    AutoLogin3,
    AutoLogin4,
    AutoTeam,
    AutoTeamSpecSpam,
    AxesIndicator,
    BackwardCompatibility,
    Ban,
    BanIp,
    BanList,
    BanUser,
    BanUserList,
    BigBrother,
    Bookmark0Address,
    Bookmark0Name,
    Bookmark0Port,
    Bookmark10Address,
    Bookmark10Name,
    Bookmark10Port,
    Bookmark1Address,
    Bookmark1Name,
    Bookmark1Port,
    Bookmark2Address,
    Bookmark2Name,
    Bookmark2Port,
    Bookmark3Address,
    Bookmark3Name,
    Bookmark3Port,
    Bookmark4Address,
    Bookmark4Name,
    Bookmark4Port,
    Bookmark5Address,
    Bookmark5Name,
    Bookmark5Port,
    Bookmark6Address,
    Bookmark6Name,
    Bookmark6Port,
    Bookmark7Address,
    Bookmark7Name,
    Bookmark7Port,
    Bookmark8Address,
    Bookmark8Name,
    Bookmark8Port,
    Bookmark9Address,
    Bookmark9Name,
    Bookmark9Port,
    BookmarkMaster0Address,
    BookmarkMaster0Name,
    BookmarkMaster0Port,
    BookmarkMaster10Address,
    BookmarkMaster10Name,
    BookmarkMaster10Port,
    BookmarkMaster1Address,
    BookmarkMaster1Name,
    BookmarkMaster1Port,
    BookmarkMaster2Address,
    BookmarkMaster2Name,
    BookmarkMaster2Port,
    BookmarkMaster3Address,
    BookmarkMaster3Name,
    BookmarkMaster3Port,
    BookmarkMaster4Address,
    BookmarkMaster4Name,
    BookmarkMaster4Port,
    BookmarkMaster5Address,
    BookmarkMaster5Name,
    BookmarkMaster5Port,
    BookmarkMaster6Address,
    BookmarkMaster6Name,
    BookmarkMaster6Port,
    BookmarkMaster7Address,
    BookmarkMaster7Name,
    BookmarkMaster7Port,
    BookmarkMaster8Address,
    BookmarkMaster8Name,
    BookmarkMaster8Port,
    BookmarkMaster9Address,
    BookmarkMaster9Name,
    BookmarkMaster9Port,
    BrakeGaugeLocx,
    BrakeGaugeLocy,
    BrakeGaugeSize,
    BugColorOverflow,
    BugRip,
    BugTransparency,
    BugTransparencyDemand,
    BugTunnel,
    Camcenter1,
    Camcenter2,
    Camcenter3,
    Camcenter4,
    CameraCustomBack,
    CameraCustomBackFromspeed,
    CameraCustomPitch,
    CameraCustomRise,
    CameraCustomRiseFromspeed,
    CameraCustomTurnSpeed,
    CameraCustomTurnSpeed180,
    CameraCustomZoom,
    CameraFollowStartX,
    CameraFollowStartY,
    CameraFollowStartZ,
    CameraForbidCustom,
    CameraForbidCustomGlance,
    CameraForbidFollow,
    CameraForbidFree,
    CameraForbidIn,
    CameraForbidServerCustom,
    CameraForbidSmart,
    CameraFreeStartX,
    CameraFreeStartY,
    CameraFreeStartZ,
    CameraGlanceBack,
    CameraGlanceBackFromspeed,
    CameraGlancePitch,
    CameraGlanceRise,
    CameraGlanceRiseFromspeed,
    CameraInTurnSpeed,
    CameraOverrideCustomGlance,
    CameraOverrideCustomGlanceServerCustom,
    CameraServerCustomBack,
    CameraServerCustomBackFromspeed,
    CameraServerCustomPitch,
    CameraServerCustomRise,
    CameraServerCustomRiseFromspeed,
    CameraServerCustomTurnSpeed,
    CameraServerCustomTurnSpeed180,
    CameraServerGlanceBack,
    CameraServerGlanceBackFromspeed,
    CameraServerGlancePitch,
    CameraServerGlanceRise,
    CameraServerGlanceRiseFromspeed,
    CameraSmartStartX,
    CameraSmartStartY,
    CameraSmartStartZ,
    CameraVisibilityClipSpeed,
    CameraVisibilityExtension,
    CameraVisibilityLowerWall,
    CameraVisibilityLowerWallSmart,
    CameraVisibilityRecoverySpeed,
    CameraVisibilitySideskew,
    CameraVisibilityWallDistance,
    Camwobble1,
    Camwobble2,
    Camwobble3,
    Camwobble4,
    Casacl,
    CenterMessage,
    ChatbotAlwaysActive,
    ChatbotDecay,
    ChatbotDelay,
    ChatbotMinTimestep,
    ChatbotNewWallBlindness,
    ChatbotRange,
    ChatterRemoveTime,
    ChatLog,
    ChatTooltip,
    CheckErrors,
    ClientPort,
    CmLocy,
    Colordepth,
    ColorB1,
    ColorB2,
    ColorB3,
    ColorB4,
    ColorG1,
    ColorG2,
    ColorG3,
    ColorG4,
    ColorR1,
    ColorR2,
    ColorR3,
    ColorR4,
    ColorStrings,
    ConnectionFloodSensitivity,
    ConnectionLimit,
    ConsoleColumns,
    ConsoleIndent,
    ConsoleLadderLog,
    ConsoleLog,
    ConsoleMessage,
    ConsoleRows,
    ConsoleRowsMax,
    CustomScreenAspect,
    CustomScreenHeight,
    CustomScreenWidth,
    CustomServerName,
    CycleAccel,
    CycleAccelEnemy,
    CycleAccelEnemyOverride,
    CycleAccelOffset,
    CycleAccelRim,
    CycleAccelRimOverride,
    CycleAccelSelf,
    CycleAccelSelfOverride,
    CycleAccelSlingshot,
    CycleAccelSlingshotOverride,
    CycleAccelTeam,
    CycleAccelTeamOverride,
    CycleAccelTunnel,
    CycleAccelTunnelOverride,
    CycleAvoidOldclientBadSync,
    CycleBlinkFrequency,
    CycleBoostfactorEnemy,
    CycleBoostfactorEnemyOverride,
    CycleBoostfactorRim,
    CycleBoostfactorRimOverride,
    CycleBoostfactorSelf,
    CycleBoostfactorSelfOverride,
    CycleBoostfactorTeam,
    CycleBoostfactorTeamOverride,
    CycleBoostEnemy,
    CycleBoostEnemyOverride,
    CycleBoostRim,
    CycleBoostRimOverride,
    CycleBoostSelf,
    CycleBoostSelfOverride,
    CycleBoostTeam,
    CycleBoostTeamOverride,
    CycleBrake,
    CycleBrakeDeplete,
    CycleBrakeDepleteOverride,
    CycleBrakeRefill,
    CycleBrakeRefillOverride,
    CycleBrakeTooltip,
    CycleDelay,
    CycleDelayDoublebindBonus,
    CycleDelayDoublebindBonusOverride,
    CycleDelayTimebased,
    CycleDelayTimebasedOverride,
    CycleDistWallShrink,
    CycleDistWallShrinkOffset,
    CycleFairAntilag,
    CycleFirstSpawnProtection,
    CycleInvulnerableTime,
    CycleMaxRefcount,
    CyclePacketlossTolerance,
    CyclePingRubber,
    CycleRubber,
    CycleRubberDelay,
    CycleRubberDelayBonus,
    CycleRubberDelayBonusOverride,
    CycleRubberDelayOverride,
    CycleRubberLegacy,
    CycleRubberMalusTurnOverride,
    CycleRubberMinadjust,
    CycleRubberMinadjustOverride,
    CycleRubberMindistance,
    CycleRubberMindistanceGap,
    CycleRubberMindistanceGapBackdoor,
    CycleRubberMindistanceGapBackdoorOverride,
    CycleRubberMindistanceGapOverride,
    CycleRubberMindistanceGapSide,
    CycleRubberMindistanceLegacy,
    CycleRubberMindistanceOverride,
    CycleRubberMindistancePreparation,
    CycleRubberMindistancePreparationOverride,
    CycleRubberMindistanceRatio,
    CycleRubberMindistanceRatioOverride,
    CycleRubberMindistanceReservoir,
    CycleRubberMindistanceReservoirOverride,
    CycleRubberMindistanceUnprepared,
    CycleRubberMindistanceUnpreparedOverride,
    CycleRubberSpeed,
    CycleRubberSpeedOverride,
    CycleRubberTime,
    CycleRubberTimebased,
    CycleRubberTimebasedOverride,
    CycleRubberTimeOverride,
    CycleRubberWallShrink,
    CycleRubberWallShrinkOverride,
    CycleSmoothMinSpeed,
    CycleSmoothThreshold,
    CycleSmoothTime,
    CycleSoundSpeed,
    CycleSpeed,
    CycleSpeedDecayAbove,
    CycleSpeedDecayAboveOverride,
    CycleSpeedDecayBelow,
    CycleSpeedDecayBelowOverride,
    CycleSpeedMax,
    CycleSpeedMaxOverride,
    CycleSpeedMin,
    CycleSpeedMinOverride,
    CycleStartSpeed,
    CycleSyncFf,
    CycleSyncFfSteps,
    CycleSyncIntervalEnemy,
    CycleSyncIntervalSelf,
    CycleTimeTolerance,
    CycleTimeToleranceOverride,
    CycleTurnLeftTooltip,
    CycleTurnMemory,
    CycleTurnRightTooltip,
    CycleTurnSpeedFactor,
    CycleTurnSpeedFactorOverride,
    CycleWallNear,
    CycleWallTime,
    CycleWidth,
    CycleWidthOverride,
    CycleWidthRubberMax,
    CycleWidthRubberMaxOverride,
    CycleWidthRubberMin,
    CycleWidthRubberMinOverride,
    CycleWidthSide,
    CycleWidthSideOverride,
    DeclareRoundWinner,
    DedicatedIdle,
    DefaultKickReason,
    DefaultKickToPort,
    DefaultKickToReason,
    DefaultKickToServer,
    DefaultShoutPlayer,
    DefaultShoutSpectator,
    DisallowRenamePlayer,
    DisallowTeamChangePlayer,
    Dither,
    DoublebindTime,
    DoublebindTimeOverride,
    EnableChat,
    EnableFriends,
    EnemyChatbotPenalty,
    EnemyCurrenttimeInfluence,
    EnemyDeadPenalty,
    EnemySuicideTimeout,
    EnemyTeammatePenalty,
    ExpectAckOnClientPlayback,
    Explosion,
    ExplosionRadius,
    ExtraRoundTime,
    FadeoutNameDelay,
    FailedAttempts,
    FastestLocx,
    FastestLocy,
    FastestSize,
    FastForwardMaxstep,
    FastForwardMaxstepReal,
    FastForwardMaxstepRel,
    FavNumPerTeamPlayer1,
    FavNumPerTeamPlayer2,
    FavNumPerTeamPlayer3,
    FavNumPerTeamPlayer4,
    FilterColorNames,
    FilterColorServerNames,
    FilterColorStrings,
    FilterColorTeam,
    FilterDarkColorNames,
    FilterDarkColorServerNames,
    FilterDarkColorStrings,
    FilterDarkColorTeam,
    FilterNameEnds,
    FilterNameMiddle,
    FinishType,
    FirstUse,
    FloorBlue,
    FloorDetail,
    FloorGreen,
    FloorMirror,
    FloorMirrorInt,
    FloorRed,
    FontSmallThresholdHeight,
    FontSmallThresholdWidth,
    ForceTurtleMode,
    FortressCollapseSpeed,
    FortressConquestDecayRate,
    FortressConquestRate,
    FortressConquestTimeout,
    FortressDefendRate,
    FortressHeldScore,
    Friend1,
    Friend10,
    Friend2,
    Friend3,
    Friend4,
    Friend5,
    Friend6,
    Friend7,
    Friend8,
    Friend9,
    Fullscreen,
    FullscreenMessage,
    GameTimeout,
    GameType,
    GlanceBackTooltip,
    GlanceLeftTooltip,
    GlanceRightTooltip,
    GlobalId,
    GlExtensions,
    GlRenderer,
    GlVendor,
    GlVersion,
    GridSize,
    GridSizeMoviepack,
    HashMethodBlacklist,
    HelpIntroductoryBlurb,
    HideIdentity1,
    HideIdentity2,
    HideIdentity3,
    HideIdentity4,
    HighRim,
    HistorySizeChat,
    HistorySizeConsole,
    HudCacheThreshold,
    HudMaxWidth,
    IdleKickTime,
    IdleRemoveTime,
    Include,
    InfinityPlane,
    IngameMenuTooltip,
    InstantChatString1_1,
    InstantChatString1_10,
    InstantChatString1_11,
    InstantChatString1_12,
    InstantChatString1_13,
    InstantChatString1_14,
    InstantChatString1_15,
    InstantChatString1_16,
    InstantChatString1_17,
    InstantChatString1_18,
    InstantChatString1_19,
    InstantChatString1_2,
    InstantChatString1_20,
    InstantChatString1_21,
    InstantChatString1_22,
    InstantChatString1_23,
    InstantChatString1_24,
    InstantChatString1_25,
    InstantChatString1_3,
    InstantChatString1_4,
    InstantChatString1_5,
    InstantChatString1_6,
    InstantChatString1_7,
    InstantChatString1_8,
    InstantChatString1_9,
    InstantChatString2_1,
    InstantChatString2_10,
    InstantChatString2_11,
    InstantChatString2_12,
    InstantChatString2_13,
    InstantChatString2_14,
    InstantChatString2_15,
    InstantChatString2_16,
    InstantChatString2_17,
    InstantChatString2_18,
    InstantChatString2_19,
    InstantChatString2_2,
    InstantChatString2_20,
    InstantChatString2_21,
    InstantChatString2_22,
    InstantChatString2_23,
    InstantChatString2_24,
    InstantChatString2_25,
    InstantChatString2_3,
    InstantChatString2_4,
    InstantChatString2_5,
    InstantChatString2_6,
    InstantChatString2_7,
    InstantChatString2_8,
    InstantChatString2_9,
    InstantChatString3_1,
    InstantChatString3_10,
    InstantChatString3_11,
    InstantChatString3_12,
    InstantChatString3_13,
    InstantChatString3_14,
    InstantChatString3_15,
    InstantChatString3_16,
    InstantChatString3_17,
    InstantChatString3_18,
    InstantChatString3_19,
    InstantChatString3_2,
    InstantChatString3_20,
    InstantChatString3_21,
    InstantChatString3_22,
    InstantChatString3_23,
    InstantChatString3_24,
    InstantChatString3_25,
    InstantChatString3_3,
    InstantChatString3_4,
    InstantChatString3_5,
    InstantChatString3_6,
    InstantChatString3_7,
    InstantChatString3_8,
    InstantChatString3_9,
    InstantChatString4_1,
    InstantChatString4_10,
    InstantChatString4_11,
    InstantChatString4_12,
    InstantChatString4_13,
    InstantChatString4_14,
    InstantChatString4_15,
    InstantChatString4_16,
    InstantChatString4_17,
    InstantChatString4_18,
    InstantChatString4_19,
    InstantChatString4_2,
    InstantChatString4_20,
    InstantChatString4_21,
    InstantChatString4_22,
    InstantChatString4_23,
    InstantChatString4_24,
    InstantChatString4_25,
    InstantChatString4_3,
    InstantChatString4_4,
    InstantChatString4_5,
    InstantChatString4_6,
    InstantChatString4_7,
    InstantChatString4_8,
    InstantChatString4_9,
    KeepPlayerSlot,
    KeepWindowActive,
    Keyboard,
    Kick,
    KickTo,
    Kill,
    LadderlogDecorateTimestamp,
    LadderlogGameTimeInterval,
    LadderlogWriteAll,
    LadderlogWriteAuthorityBlurb,
    LadderlogWriteBasezoneConquered,
    LadderlogWriteBasezoneConquerer,
    LadderlogWriteChat,
    LadderlogWriteDeathFrag,
    LadderlogWriteDeathSuicide,
    LadderlogWriteDeathTeamkill,
    LadderlogWriteEncoding,
    LadderlogWriteGameEnd,
    LadderlogWriteGameTime,
    LadderlogWriteMatchWinner,
    LadderlogWriteNewMatch,
    LadderlogWriteNewRound,
    LadderlogWriteNumHumans,
    LadderlogWriteOnlinePlayer,
    LadderlogWritePlayerEntered,
    LadderlogWritePlayerLeft,
    LadderlogWritePlayerRenamed,
    LadderlogWritePositions,
    LadderlogWriteRoundScore,
    LadderlogWriteRoundScoreTeam,
    LadderlogWriteRoundWinner,
    LadderlogWriteSacrifice,
    LadderlogWriteTeamCreated,
    LadderlogWriteTeamDestroyed,
    LadderlogWriteTeamPlayerAdded,
    LadderlogWriteTeamPlayerRemoved,
    LadderlogWriteTeamRenamed,
    LadderlogWriteWaitForExternalScript,
    LadderGainExtra,
    LadderLoseMinOnLoad,
    LadderLosePercentOnLoad,
    LadderMinBet,
    LadderPercentBet,
    LadderTax,
    LagCredit,
    LagCreditSingle,
    LagCreditTime,
    LagCreditVariance,
    LagFastTime,
    LagFastWeight,
    LagFrequencyThreshold,
    LagMaxSpeedupTimer,
    LagOffsetClient,
    LagOffsetLegacy,
    LagOffsetServer,
    LagOMeter,
    LagOMeterBlend,
    LagOMeterScale,
    LagOMeterThreshold,
    LagOMeterUseOld,
    LagSlowTime,
    LagSlowWeight,
    LagSweetSpot,
    LagThreshold,
    LanguageFirst,
    LanguageSecond,
    LastChatBreakTime,
    LastCheckErrors,
    LastColordepth,
    LastFullscreen,
    LastZdepth,
    LegacyLogNames,
    LimitRounds,
    LimitScore,
    LimitTime,
    LocalTeam,
    LocalUser,
    LowerSky,
    MapFile,
    MapFileOverride,
    MapUri,
    MaxClients,
    MaxClientsSameIpHard,
    MaxClientsSameIpSoft,
    MaxInRate,
    MaxOutRate,
    MaxPlayersSameIp,
    MaxProtocolVersion,
    MaxVotes,
    MaxVotesPerVoter,
    Md5Prefix,
    Md5Suffix,
    MessageOfDay,
    MessageOfDayTimeout,
    MinPlayers,
    MinPlayTimeOnline,
    MinPlayTimeTeam,
    MinPlayTimeTotal,
    MinProtocolVersion,
    MinVoters,
    MouseGrab,
    MoveTo,
    Moviepack,
    MoviepackFloorBlue,
    MoviepackFloorGreen,
    MoviepackFloorRed,
    MoviepackRimWallStretchX,
    MoviepackRimWallStretchY,
    MoviepackWallStretch,
    NameTeamAfterPlayer1,
    NameTeamAfterPlayer2,
    NameTeamAfterPlayer3,
    NameTeamAfterPlayer4,
    NetworkAutobanFactor,
    NetworkAutobanMaxKph,
    NetworkAutobanOffset,
    NetworkMinBan,
    NetworkSpectatorTime,
    NewFeatureDelay,
    NewTeamAllowed,
    NumAis,
    Password,
    PasswordStorage,
    PingCharity,
    PingCharityMax,
    PingCharityMin,
    PingCharityServer,
    PingFloodGlobal,
    PingFloodTime10,
    PingFloodTime100,
    PingFloodTime20,
    PingFloodTime50,
    PingLocx,
    PingLocy,
    PingSize,
    Players,
    Player1,
    Player2,
    Player3,
    Player4,
    PlayerChatWaitFraction,
    PlayerChatWaitMax,
    PlayerChatWaitSingle,
    PlayerChatWaitTeamleader,
    PlayerListHiddenPlayerPrefix,
    PlayerMessage,
    PlayerRandomColor,
    PlayTimeOnline,
    PlayTimeTeam,
    PlayTimeTotal,
    PngScreenshot,
    PredictObjects,
    PrefixSpamEnable,
    PrefixSpamLengthMultiplier,
    PrefixSpamNumberColorCodesMultiplier,
    PrefixSpamNumberKnownPrefixesMultiplier,
    PrefixSpamRequiredScore,
    PrefixSpamStartColorMultiplier,
    PrefixSpamTimeoutMultiplier,
    ProtectSensitiveFiles,
    RealArenaSizeFactor,
    RealCycleSpeedFactor,
    RecordingDebuglevel,
    RecordTurtleMode,
    RemoveHelpTopic,
    Rename,
    ReserveScreenName,
    ResourceRepositoryClient,
    ResourceRepositoryServer,
    RimWallStretchX,
    RimWallStretchY,
    RimWallWrapY,
    Rinclude,
    RoundCenterMessage,
    RoundConsoleMessage,
    RubberGaugeLocx,
    RubberGaugeLocy,
    RubberGaugeSize,
    SavedInVersion,
    Say,
    ScoreDeathzone,
    ScoreDie,
    ScoreHole,
    ScoreKill,
    ScoreLocx,
    ScoreLocy,
    ScoreSize,
    ScoreSuicide,
    ScoreSurvive,
    ScoreWin,
    ServerDns,
    ServerIp,
    ServerName,
    ServerPort,
    SettingLegacyBehaviorAnnoying,
    SettingLegacyBehaviorBreaking,
    SettingLegacyBehaviorBumpy,
    SettingLegacyBehaviorCheating,
    SettingLegacyBehaviorVisual,
    ShowAlive,
    ShowBrake,
    ShowFastest,
    ShowFps,
    ShowHud,
    ShowOwnIp,
    ShowOwnName,
    ShowPing,
    ShowRecordingTime,
    ShowRubber,
    ShowScore,
    ShowSpeed,
    ShowTime,
    ShowTime24,
    ShuffleSpamMessagesPerRound,
    Silence,
    SilenceDefault,
    SimpleTrail,
    Sinclude,
    SizeFactor,
    SkyWobble,
    Slap,
    SmartGlanceCustom1,
    SmartGlanceCustom2,
    SmartGlanceCustom3,
    SmartGlanceCustom4,
    SmoothShading,
    SoftwareRenderer,
    SoundBufferShift,
    SoundQuality,
    SoundSources,
    SpamAutokick,
    SpamAutokickCount,
    SpamMaxlen,
    SpamMaxlenOverride,
    SpamPenalty,
    SpamProtection,
    SpamProtectionChat,
    SpamProtectionRepeat,
    SpamProtectionVote,
    Sparks,
    SpawnWingmenBack,
    SpawnWingmenSide,
    SpectatorMode1,
    SpectatorMode2,
    SpectatorMode3,
    SpectatorMode4,
    SpeedFactor,
    SpeedGaugeLocx,
    SpeedGaugeLocy,
    SpeedGaugeSize,
    SpAiIq,
    SpAutoAis,
    SpAutoIq,
    SpExplosionRadius,
    SpFinishType,
    SpGameType,
    SpLimitRounds,
    SpLimitScore,
    SpLimitTime,
    SpMinPlayers,
    SpNumAis,
    SpScoreWin,
    SpSizeFactor,
    SpSpeedFactor,
    SpTeamsMax,
    SpTeamsMin,
    SpTeamBalanceOnQuit,
    SpTeamBalanceWithAis,
    SpTeamMaxImbalance,
    SpTeamMaxPlayers,
    SpTeamMinPlayers,
    SpWallsLength,
    SpWallsStayUpDelay,
    SpWinZoneMinLastDeath,
    SpWinZoneMinRoundTime,
    StartCam1,
    StartCam2,
    StartCam3,
    StartCam4,
    StartFov1,
    StartFov2,
    StartFov3,
    StartFov4,
    StartNewMatch,
    StopRecording,
    Suspend,
    SuspendDefaultRounds,
    SwapMode,
    SwitchViewTooltip,
    TalkToMaster,
    Teams,
    TeamsMax,
    TeamsMin,
    TeamBalanceOnQuit,
    TeamBalanceWithAis,
    TeamBlue1,
    TeamBlue2,
    TeamBlue3,
    TeamBlue4,
    TeamBlue5,
    TeamBlue6,
    TeamBlue7,
    TeamBlue8,
    TeamCenterIsBoss,
    TeamEliminationMode,
    TeamGreen1,
    TeamGreen2,
    TeamGreen3,
    TeamGreen4,
    TeamGreen5,
    TeamGreen6,
    TeamGreen7,
    TeamGreen8,
    TeamMaxImbalance,
    TeamMaxPlayers,
    TeamMinPlayers,
    TeamName1,
    TeamName2,
    TeamName3,
    TeamName4,
    TeamName5,
    TeamName6,
    TeamName7,
    TeamName8,
    TeamRed1,
    TeamRed2,
    TeamRed3,
    TeamRed4,
    TeamRed5,
    TeamRed6,
    TeamRed7,
    TeamRed8,
    TexturesHi,
    TextureMode0,
    TextureMode1,
    TextureMode2,
    TextureMode3,
    TextOut,
    TimebotActionHigh,
    TimebotActionMax,
    TimebotActionMedium,
    TimebotKickSeverity,
    TimebotSensitivity,
    TitleOfDay,
    ToggleSpectatorTooltip,
    TopologyPolice,
    TopologyPoliceParallel,
    TrustLan,
    UnbanIp,
    UnbanUser,
    UnlockAllTeams,
    Unsilence,
    Unsuspend,
    UpperSky,
    Url,
    User1,
    User2,
    User3,
    User4,
    UserAlias,
    UserLevel,
    UserRemove,
    UseDisplaylists,
    ViewportConf,
    ViewportToPlayer1,
    ViewportToPlayer2,
    ViewportToPlayer3,
    ViewportToPlayer4,
    Voice,
    VotesCancel,
    VotesSuspend,
    VotesSuspendDefault,
    VotesUnsuspend,
    VoteKickReason,
    VoteKickToPort,
    VoteKickToServer,
    VoteUseServerControlledKick,
    VotingBias,
    VotingBiasCommand,
    VotingBiasInclude,
    VotingBiasKick,
    VotingBiasSilence,
    VotingBiasSuspend,
    VotingBiasVoice,
    VotingDecay,
    VotingHarmTime,
    VotingKickMinharm,
    VotingKickTime,
    VotingMaturity,
    VotingPrivacy,
    VotingSpamIssue,
    VotingSpamReject,
    VotingStartDecay,
    VotingSuspendRounds,
    VotingTimeout,
    VotingTimeoutPerVoter,
    WaitForExternalScript,
    WaitForExternalScriptTimeout,
    WallsLength,
    WallsStayUpDelay,
    WhitelistEnemiesIp,
    WhitelistEnemiesUsername,
    WhiteSparks,
    WinZoneDeaths,
    WinZoneExpansion,
    WinZoneInitialSize,
    WinZoneMinLastDeath,
    WinZoneMinRoundTime,
    WinZoneRandomness,
    WordDelimiters,
    WrapMenu,
    Zdepth,
    ZoneAlpha,
    ZoneAlphaServer,
    ZoneAlphaToggle,
    ZoneBottom,
    ZoneHeight,
    ZoneSegments,
    ZoneSegLength,
    #[cfg(feature = "ap")]
    AccelzoneRate,
    #[cfg(feature = "ap")]
    AccessLevelAdmin,
    #[cfg(feature = "ap")]
    AccessLevelOp,
    #[cfg(feature = "ap")]
    AccessLevelOpMax,
    #[cfg(feature = "ap")]
    AccessLevelOpMin,
    #[cfg(feature = "ap")]
    AccessLevelQueueConfigs,
    #[cfg(feature = "ap")]
    AccessLevelQueueMaps,
    #[cfg(feature = "ap")]
    AccessLevelReportsClear,
    #[cfg(feature = "ap")]
    AccessLevelReportsRead,
    #[cfg(feature = "ap")]
    AccessLevelSpyConsole,
    #[cfg(feature = "ap")]
    AccessLevelSubstitute,
    #[cfg(feature = "ap")]
    AccessLevelTeam,
    #[cfg(feature = "ap")]
    AccessLevelViewChats,
    #[cfg(feature = "ap")]
    AddScorePlayer,
    #[cfg(feature = "ap")]
    AddScoreTeam,
    #[cfg(feature = "ap")]
    AddZoneIdRoute,
    #[cfg(feature = "ap")]
    AddZoneRoute,
    #[cfg(feature = "ap")]
    AdminKillMessage,
    #[cfg(feature = "ap")]
    AdminLog,
    #[cfg(feature = "ap")]
    AdminName,
    #[cfg(feature = "ap")]
    AiBypass,
    #[cfg(feature = "ap")]
    AiReload,
    #[cfg(feature = "ap")]
    AllowTeamNameLeader,
    #[cfg(feature = "ap")]
    Announce,
    #[cfg(feature = "ap")]
    ApplyRotation,
    #[cfg(feature = "ap")]
    ApplySpecForce,
    #[cfg(feature = "ap")]
    ApplyTeamForce,
    #[cfg(feature = "ap")]
    ArenaBoundary,
    #[cfg(feature = "ap")]
    ArenaBoundaryKill,
    #[cfg(feature = "ap")]
    AutoSubstitution,
    #[cfg(feature = "ap")]
    BallsInteract,
    #[cfg(feature = "ap")]
    BallAutorespawn,
    #[cfg(feature = "ap")]
    BallCycleAccelBoost,
    #[cfg(feature = "ap")]
    BallKills,
    #[cfg(feature = "ap")]
    BallSpeedDecay,
    #[cfg(feature = "ap")]
    BallSpeedHitDecay,
    #[cfg(feature = "ap")]
    BallTeamMode,
    #[cfg(feature = "ap")]
    BannedWords,
    #[cfg(feature = "ap")]
    BannedWordsAdd,
    #[cfg(feature = "ap")]
    BannedWordsDelimiters,
    #[cfg(feature = "ap")]
    BannedWordsList,
    #[cfg(feature = "ap")]
    BannedWordsOptions,
    #[cfg(feature = "ap")]
    BannedWordsRemove,
    #[cfg(feature = "ap")]
    BannedWordsWhole,
    #[cfg(feature = "ap")]
    BaseEnemyKill,
    #[cfg(feature = "ap")]
    BaseEnemyRespawn,
    #[cfg(feature = "ap")]
    BaseRespawn,
    #[cfg(feature = "ap")]
    BaseRespawnRemindTime,
    #[cfg(feature = "ap")]
    Boot,
    #[cfg(feature = "ap")]
    CameraForbidMer,
    #[cfg(feature = "ap")]
    CameraGlanceForwardSnap,
    #[cfg(feature = "ap")]
    CameraGlanceHold,
    #[cfg(feature = "ap")]
    CameraGlanceSnap,
    #[cfg(feature = "ap")]
    CenterPlayerMessage,
    #[cfg(feature = "ap")]
    CfgUserSave,
    #[cfg(feature = "ap")]
    ChatbotControlledByServer,
    #[cfg(feature = "ap")]
    ChatbotEnabled,
    #[cfg(feature = "ap")]
    ChatlogWriteTeam,
    #[cfg(feature = "ap")]
    ChattersKill,
    #[cfg(feature = "ap")]
    ChattersList,
    #[cfg(feature = "ap")]
    ChattersSilence,
    #[cfg(feature = "ap")]
    ChattersSuspend,
    #[cfg(feature = "ap")]
    ChatLogColors,
    #[cfg(feature = "ap")]
    ClearChatlog,
    #[cfg(feature = "ap")]
    ClearLadderlog,
    #[cfg(feature = "ap")]
    ClearReports,
    #[cfg(feature = "ap")]
    ClearScorelog,
    #[cfg(feature = "ap")]
    ClientDownloadSettings,
    #[cfg(feature = "ap")]
    CollapseAll,
    #[cfg(feature = "ap")]
    CollapseZone,
    #[cfg(feature = "ap")]
    CollapseZoneId,
    #[cfg(feature = "ap")]
    ColorDeathzoneBlue,
    #[cfg(feature = "ap")]
    ColorDeathzoneGreen,
    #[cfg(feature = "ap")]
    ColorDeathzoneRed,
    #[cfg(feature = "ap")]
    ColorRubberzoneBlue,
    #[cfg(feature = "ap")]
    ColorRubberzoneGreen,
    #[cfg(feature = "ap")]
    ColorRubberzoneRed,
    #[cfg(feature = "ap")]
    ColorTeleportzoneBlue,
    #[cfg(feature = "ap")]
    ColorTeleportzoneGreen,
    #[cfg(feature = "ap")]
    ColorTeleportzoneRed,
    #[cfg(feature = "ap")]
    ColorWinzoneBlue,
    #[cfg(feature = "ap")]
    ColorWinzoneGreen,
    #[cfg(feature = "ap")]
    ColorWinzoneRed,
    #[cfg(feature = "ap")]
    CondenseConquestOutput,
    #[cfg(feature = "ap")]
    ConfigRotation,
    #[cfg(feature = "ap")]
    ConfigRotationAdd,
    #[cfg(feature = "ap")]
    ConfigRotationLoad,
    #[cfg(feature = "ap")]
    ConfigRotationRemove,
    #[cfg(feature = "ap")]
    ConfigRotationSet,
    #[cfg(feature = "ap")]
    ConfigRotationType,
    #[cfg(feature = "ap")]
    ConfigStorage,
    #[cfg(feature = "ap")]
    ConsoleDecorateId,
    #[cfg(feature = "ap")]
    ConsoleDecorateIp,
    #[cfg(feature = "ap")]
    ConsoleDecorateTimestamp,
    #[cfg(feature = "ap")]
    ConsoleLogColor,
    #[cfg(feature = "ap")]
    ConsoleLogColorDecorateTimestamp,
    #[cfg(feature = "ap")]
    CustomAuthority,
    #[cfg(feature = "ap")]
    CustomAuthorityConnection,
    #[cfg(feature = "ap")]
    CustomAuthorityEnabled,
    #[cfg(feature = "ap")]
    CustomCenterMessage,
    #[cfg(feature = "ap")]
    CustomCenterPlayerMessage,
    #[cfg(feature = "ap")]
    CustomConfigs,
    #[cfg(feature = "ap")]
    CustomInvalidCommands,
    #[cfg(feature = "ap")]
    CustomMessage,
    #[cfg(feature = "ap")]
    CustomPlayerCenterMessage,
    #[cfg(feature = "ap")]
    CustomPlayerMessage,
    #[cfg(feature = "ap")]
    CustomShorthandAdd,
    #[cfg(feature = "ap")]
    CustomShorthandAllowed,
    #[cfg(feature = "ap")]
    CustomShorthandEnabled,
    #[cfg(feature = "ap")]
    CustomShorthandLinksList,
    #[cfg(feature = "ap")]
    CustomShorthandList,
    #[cfg(feature = "ap")]
    CustomShorthandRemove,
    #[cfg(feature = "ap")]
    CycleDeathTeleport,
    #[cfg(feature = "ap")]
    CycleDeathTeleportExplosion,
    #[cfg(feature = "ap")]
    CycleDeathTeleportReset,
    #[cfg(feature = "ap")]
    CycleDelayBonus,
    #[cfg(feature = "ap")]
    CycleRespawnZone,
    #[cfg(feature = "ap")]
    CycleRespawnZoneEnemy,
    #[cfg(feature = "ap")]
    CycleRespawnZoneEnemyKill,
    #[cfg(feature = "ap")]
    CycleRespawnZoneGrowth,
    #[cfg(feature = "ap")]
    CycleRespawnZoneRadius,
    #[cfg(feature = "ap")]
    CycleRespawnZoneRespawn,
    #[cfg(feature = "ap")]
    CycleRespawnZoneType,
    #[cfg(feature = "ap")]
    CycleRubberDepleteEnemy,
    #[cfg(feature = "ap")]
    CycleRubberDepleteEnemyOverride,
    #[cfg(feature = "ap")]
    CycleRubberDepleteRim,
    #[cfg(feature = "ap")]
    CycleRubberDepleteRimOverride,
    #[cfg(feature = "ap")]
    CycleRubberDepleteSelf,
    #[cfg(feature = "ap")]
    CycleRubberDepleteSelfOverride,
    #[cfg(feature = "ap")]
    CycleRubberDepleteTeam,
    #[cfg(feature = "ap")]
    CycleRubberDepleteTeamOverride,
    #[cfg(feature = "ap")]
    CycleTurn,
    #[cfg(feature = "ap")]
    CycleZonesAim,
    #[cfg(feature = "ap")]
    CycleZonesAimChatbot,
    #[cfg(feature = "ap")]
    CycleZonesAimTypes,
    #[cfg(feature = "ap")]
    CycleZonesApproach,
    #[cfg(feature = "ap")]
    CycleZonesApproch,
    #[cfg(feature = "ap")]
    CycleZonesAvoid,
    #[cfg(feature = "ap")]
    CycleZonesAvoidAimOrder,
    #[cfg(feature = "ap")]
    CycleZonesAvoidChatbot,
    #[cfg(feature = "ap")]
    CycleZonesAvoidOld,
    #[cfg(feature = "ap")]
    CycleZonesAvoidRange,
    #[cfg(feature = "ap")]
    DeadlyExplosions,
    #[cfg(feature = "ap")]
    DeathzoneRandomColors,
    #[cfg(feature = "ap")]
    DeathzoneRotation,
    #[cfg(feature = "ap")]
    DeathzoneRotationSpeed,
    #[cfg(feature = "ap")]
    DeathShot,
    #[cfg(feature = "ap")]
    DedicatedFps,
    #[cfg(feature = "ap")]
    DedicatedFpsIdleFactor,
    #[cfg(feature = "ap")]
    DefaultMapFile,
    #[cfg(feature = "ap")]
    DefaultMapFileOnEmpty,
    #[cfg(feature = "ap")]
    DelayCommand,
    #[cfg(feature = "ap")]
    DelayCommandClear,
    #[cfg(feature = "ap")]
    DelayCommandRemove,
    #[cfg(feature = "ap")]
    Deop,
    #[cfg(feature = "ap")]
    DestroyAll,
    #[cfg(feature = "ap")]
    DestroyZone,
    #[cfg(feature = "ap")]
    DestroyZoneId,
    #[cfg(feature = "ap")]
    DisplayScoresDuringChat,
    #[cfg(feature = "ap")]
    EnableFriendsCasing,
    #[cfg(feature = "ap")]
    Exit,
    #[cfg(feature = "ap")]
    FlagBlinkEnd,
    #[cfg(feature = "ap")]
    FlagBlinkEstimatePosition,
    #[cfg(feature = "ap")]
    FlagBlinkOnTime,
    #[cfg(feature = "ap")]
    FlagBlinkStart,
    #[cfg(feature = "ap")]
    FlagBlinkTime,
    #[cfg(feature = "ap")]
    FlagBlinkTrackTime,
    #[cfg(feature = "ap")]
    FlagChatBlinkTime,
    #[cfg(feature = "ap")]
    FlagColorB,
    #[cfg(feature = "ap")]
    FlagColorG,
    #[cfg(feature = "ap")]
    FlagColorR,
    #[cfg(feature = "ap")]
    FlagConquestWinsRound,
    #[cfg(feature = "ap")]
    FlagControls,
    #[cfg(feature = "ap")]
    FlagDropHome,
    #[cfg(feature = "ap")]
    FlagDropTime,
    #[cfg(feature = "ap")]
    FlagHoldScore,
    #[cfg(feature = "ap")]
    FlagHoldScoreTime,
    #[cfg(feature = "ap")]
    FlagHoldTime,
    #[cfg(feature = "ap")]
    FlagHoldTimeDrop,
    #[cfg(feature = "ap")]
    FlagHomeRandomnessX,
    #[cfg(feature = "ap")]
    FlagHomeRandomnessY,
    #[cfg(feature = "ap")]
    FlagPassDistance,
    #[cfg(feature = "ap")]
    FlagPassMode,
    #[cfg(feature = "ap")]
    FlagPassSpeed,
    #[cfg(feature = "ap")]
    FlagRequiredHome,
    #[cfg(feature = "ap")]
    FlagTeam,
    #[cfg(feature = "ap")]
    ForbidHudMap,
    #[cfg(feature = "ap")]
    ForceRespawnScript,
    #[cfg(feature = "ap")]
    FullscreenPlayerMessage,
    #[cfg(feature = "ap")]
    GameSpHumans,
    #[cfg(feature = "ap")]
    GameWaitPlayersEnabled,
    #[cfg(feature = "ap")]
    GetCurrentMap,
    #[cfg(feature = "ap")]
    GivePoints,
    #[cfg(feature = "ap")]
    GlanceForwardTooltip,
    #[cfg(feature = "ap")]
    GoalRoundEnd,
    #[cfg(feature = "ap")]
    Help,
    #[cfg(feature = "ap")]
    HelpMessage,
    #[cfg(feature = "ap")]
    HelpMessageType,
    #[cfg(feature = "ap")]
    IdleKickExempt,
    #[cfg(feature = "ap")]
    InterceptCommands,
    #[cfg(feature = "ap")]
    InterceptUnknownCommands,
    #[cfg(feature = "ap")]
    KillAll,
    #[cfg(feature = "ap")]
    KillAllScripts,
    #[cfg(feature = "ap")]
    KillId,
    #[cfg(feature = "ap")]
    KillScript,
    #[cfg(feature = "ap")]
    KohScore,
    #[cfg(feature = "ap")]
    KohScoreTime,
    #[cfg(feature = "ap")]
    LadderlogEnabled,
    #[cfg(feature = "ap")]
    LadderlogObjectzonePlayerEnteredInside,
    #[cfg(feature = "ap")]
    LadderlogObjectzoneZoneEnteredPollrate,
    #[cfg(feature = "ap")]
    LadderlogWriteAdminCommand,
    #[cfg(feature = "ap")]
    LadderlogWriteAdminLogin,
    #[cfg(feature = "ap")]
    LadderlogWriteAdminLogout,
    #[cfg(feature = "ap")]
    LadderlogWriteAiPositions,
    #[cfg(feature = "ap")]
    LadderlogWriteBallVanish,
    #[cfg(feature = "ap")]
    LadderlogWriteBasezoneConquererTeam,
    #[cfg(feature = "ap")]
    LadderlogWriteBaseEnemyRespawn,
    #[cfg(feature = "ap")]
    LadderlogWriteBaseRespawn,
    #[cfg(feature = "ap")]
    LadderlogWriteBlastzonePlayerEnter,
    #[cfg(feature = "ap")]
    LadderlogWriteCommand,
    #[cfg(feature = "ap")]
    LadderlogWriteCurrentMap,
    #[cfg(feature = "ap")]
    LadderlogWriteCustomInvalidCommand,
    #[cfg(feature = "ap")]
    LadderlogWriteCycleCreated,
    #[cfg(feature = "ap")]
    LadderlogWriteCycleDeathTeleport,
    #[cfg(feature = "ap")]
    LadderlogWriteCycleDestroyed,
    #[cfg(feature = "ap")]
    LadderlogWriteDeathzoneActivated,
    #[cfg(feature = "ap")]
    LadderlogWriteDeathBasezoneConquered,
    #[cfg(feature = "ap")]
    LadderlogWriteDeathDeathshot,
    #[cfg(feature = "ap")]
    LadderlogWriteDeathDeathzone,
    #[cfg(feature = "ap")]
    LadderlogWriteDeathDeathzoneTeam,
    #[cfg(feature = "ap")]
    LadderlogWriteDeathExplosion,
    #[cfg(feature = "ap")]
    LadderlogWriteDeathRubberzone,
    #[cfg(feature = "ap")]
    LadderlogWriteDeathSelfDestruct,
    #[cfg(feature = "ap")]
    LadderlogWriteDeathShotFrag,
    #[cfg(feature = "ap")]
    LadderlogWriteDeathShotSuicide,
    #[cfg(feature = "ap")]
    LadderlogWriteDeathShotTeamkill,
    #[cfg(feature = "ap")]
    LadderlogWriteDeathZombiezone,
    #[cfg(feature = "ap")]
    LadderlogWriteEndChallenge,
    #[cfg(feature = "ap")]
    LadderlogWriteFlagConquestRoundWin,
    #[cfg(feature = "ap")]
    LadderlogWriteFlagDrop,
    #[cfg(feature = "ap")]
    LadderlogWriteFlagHeld,
    #[cfg(feature = "ap")]
    LadderlogWriteFlagReturn,
    #[cfg(feature = "ap")]
    LadderlogWriteFlagScore,
    #[cfg(feature = "ap")]
    LadderlogWriteFlagTake,
    #[cfg(feature = "ap")]
    LadderlogWriteFlagTimeout,
    #[cfg(feature = "ap")]
    LadderlogWriteInvalidCommand,
    #[cfg(feature = "ap")]
    LadderlogWriteMatchEnded,
    #[cfg(feature = "ap")]
    LadderlogWriteMatchScore,
    #[cfg(feature = "ap")]
    LadderlogWriteMatchScoreTeam,
    #[cfg(feature = "ap")]
    LadderlogWriteNewSet,
    #[cfg(feature = "ap")]
    LadderlogWriteNextRound,
    #[cfg(feature = "ap")]
    LadderlogWriteObjectzonePlayerEntered,
    #[cfg(feature = "ap")]
    LadderlogWriteObjectzonePlayerLeft,
    #[cfg(feature = "ap")]
    LadderlogWriteObjectzoneSpawned,
    #[cfg(feature = "ap")]
    LadderlogWriteObjectzoneZoneEntered,
    #[cfg(feature = "ap")]
    LadderlogWriteObjectzoneZoneLeft,
    #[cfg(feature = "ap")]
    LadderlogWriteOnlineAi,
    #[cfg(feature = "ap")]
    LadderlogWriteOnlinePlayersAlive,
    #[cfg(feature = "ap")]
    LadderlogWriteOnlinePlayersCount,
    #[cfg(feature = "ap")]
    LadderlogWriteOnlinePlayersDead,
    #[cfg(feature = "ap")]
    LadderlogWriteOnlineTeam,
    #[cfg(feature = "ap")]
    LadderlogWriteOnlineZone,
    #[cfg(feature = "ap")]
    LadderlogWritePlayerAiEntered,
    #[cfg(feature = "ap")]
    LadderlogWritePlayerAiLeft,
    #[cfg(feature = "ap")]
    LadderlogWritePlayerColoredName,
    #[cfg(feature = "ap")]
    LadderlogWritePlayerEnteredGrid,
    #[cfg(feature = "ap")]
    LadderlogWritePlayerEnteredSpectator,
    #[cfg(feature = "ap")]
    LadderlogWritePlayerGridpos,
    #[cfg(feature = "ap")]
    LadderlogWritePlayerJoinsSpectators,
    #[cfg(feature = "ap")]
    LadderlogWritePlayerKilled,
    #[cfg(feature = "ap")]
    LadderlogWritePlayerLeavesSpectators,
    #[cfg(feature = "ap")]
    LadderlogWritePlayerLogin,
    #[cfg(feature = "ap")]
    LadderlogWritePlayerLogout,
    #[cfg(feature = "ap")]
    LadderlogWriteQueueFinished,
    #[cfg(feature = "ap")]
    LadderlogWriteQueueStarted,
    #[cfg(feature = "ap")]
    LadderlogWriteRoundCommencing,
    #[cfg(feature = "ap")]
    LadderlogWriteRoundEnded,
    #[cfg(feature = "ap")]
    LadderlogWriteRoundFinished,
    #[cfg(feature = "ap")]
    LadderlogWriteRoundStarted,
    #[cfg(feature = "ap")]
    LadderlogWriteSetWinner,
    #[cfg(feature = "ap")]
    LadderlogWriteShutdown,
    #[cfg(feature = "ap")]
    LadderlogWriteSoccerBallPlayerEntered,
    #[cfg(feature = "ap")]
    LadderlogWriteSoccerGoalPlayerEntered,
    #[cfg(feature = "ap")]
    LadderlogWriteSoccerGoalScored,
    #[cfg(feature = "ap")]
    LadderlogWriteSpawnPositionTeam,
    #[cfg(feature = "ap")]
    LadderlogWriteStartChallenge,
    #[cfg(feature = "ap")]
    LadderlogWriteSvgCreated,
    #[cfg(feature = "ap")]
    LadderlogWriteTacticalPosition,
    #[cfg(feature = "ap")]
    LadderlogWriteTacticalStatistics,
    #[cfg(feature = "ap")]
    LadderlogWriteTargetzoneConquered,
    #[cfg(feature = "ap")]
    LadderlogWriteTargetzonePlayerEnter,
    #[cfg(feature = "ap")]
    LadderlogWriteTargetzonePlayerLeft,
    #[cfg(feature = "ap")]
    LadderlogWriteTargetzoneTimeout,
    #[cfg(feature = "ap")]
    LadderlogWriteTeamColoredName,
    #[cfg(feature = "ap")]
    LadderlogWriteVoter,
    #[cfg(feature = "ap")]
    LadderlogWriteVoteCreated,
    #[cfg(feature = "ap")]
    LadderlogWriteWinzoneActivated,
    #[cfg(feature = "ap")]
    LadderlogWriteWinzonePlayerEnter,
    #[cfg(feature = "ap")]
    LadderlogWriteZoneCollapsed,
    #[cfg(feature = "ap")]
    LadderlogWriteZoneCreated,
    #[cfg(feature = "ap")]
    LadderlogWriteZoneGridpos,
    #[cfg(feature = "ap")]
    LadderlogWriteZoneRouteStopped,
    #[cfg(feature = "ap")]
    LadderlogWriteZoneShotReleased,
    #[cfg(feature = "ap")]
    LadderlogWriteZoneSpawned,
    #[cfg(feature = "ap")]
    LadderHighscoreOutput,
    #[cfg(feature = "ap")]
    LanguageReload,
    #[cfg(feature = "ap")]
    LegacyLadderlogCommand,
    #[cfg(feature = "ap")]
    LimitAdvance,
    #[cfg(feature = "ap")]
    LimitScoreMinLead,
    #[cfg(feature = "ap")]
    LimitSets,
    #[cfg(feature = "ap")]
    ListAllCommands,
    #[cfg(feature = "ap")]
    ListAllCommandsLevels,
    #[cfg(feature = "ap")]
    ListScripts,
    #[cfg(feature = "ap")]
    LoadCustomConfigs,
    #[cfg(feature = "ap")]
    Login,
    #[cfg(feature = "ap")]
    Logout,
    #[cfg(feature = "ap")]
    LogPlayersActivities,
    #[cfg(feature = "ap")]
    LogTurns,
    #[cfg(feature = "ap")]
    LogTurnsTimestamp,
    #[cfg(feature = "ap")]
    LogTurnsWinner,
    #[cfg(feature = "ap")]
    LogZoneGridpos,
    #[cfg(feature = "ap")]
    LogZoneGridposId,
    #[cfg(feature = "ap")]
    MapOnchangeInclude,
    #[cfg(feature = "ap")]
    MapRotation,
    #[cfg(feature = "ap")]
    MapRotationAdd,
    #[cfg(feature = "ap")]
    MapRotationLoad,
    #[cfg(feature = "ap")]
    MapRotationRemove,
    #[cfg(feature = "ap")]
    MapRotationSet,
    #[cfg(feature = "ap")]
    MapStorage,
    #[cfg(feature = "ap")]
    MegaShotDir,
    #[cfg(feature = "ap")]
    MegaShotExplosion,
    #[cfg(feature = "ap")]
    MegaShotMult,
    #[cfg(feature = "ap")]
    MegaShotThresh,
    #[cfg(feature = "ap")]
    MinFlagsHome,
    #[cfg(feature = "ap")]
    MoveHere,
    #[cfg(feature = "ap")]
    NumAisPerRound,
    #[cfg(feature = "ap")]
    OnlineStatsInterval,
    #[cfg(feature = "ap")]
    Op,
    #[cfg(feature = "ap")]
    PlayerCenterMessage,
    #[cfg(feature = "ap")]
    PlayerFullscreenMessage,
    #[cfg(feature = "ap")]
    PlayerGridposInterval,
    #[cfg(feature = "ap")]
    PlayerGridposOnTurn,
    #[cfg(feature = "ap")]
    PlayerUniqueColor,
    #[cfg(feature = "ap")]
    PortMax,
    #[cfg(feature = "ap")]
    PortMin,
    #[cfg(feature = "ap")]
    PredictWalls,
    #[cfg(feature = "ap")]
    QueuersList,
    #[cfg(feature = "ap")]
    QueueConfig,
    #[cfg(feature = "ap")]
    QueueEnabled,
    #[cfg(feature = "ap")]
    QueueGive,
    #[cfg(feature = "ap")]
    QueueIncrement,
    #[cfg(feature = "ap")]
    QueueLimit,
    #[cfg(feature = "ap")]
    QueueLimitEnabled,
    #[cfg(feature = "ap")]
    QueueLimitExcempt,
    #[cfg(feature = "ap")]
    QueueLog,
    #[cfg(feature = "ap")]
    QueueMap,
    #[cfg(feature = "ap")]
    QueueMax,
    #[cfg(feature = "ap")]
    QueueRefill,
    #[cfg(feature = "ap")]
    QueueRefillActive,
    #[cfg(feature = "ap")]
    QueueRefillTime,
    #[cfg(feature = "ap")]
    Quit,
    #[cfg(feature = "ap")]
    RaceChances,
    #[cfg(feature = "ap")]
    RaceCheckpointCountdown,
    #[cfg(feature = "ap")]
    RaceCheckpointLaps,
    #[cfg(feature = "ap")]
    RaceCheckpointRequireHit,
    #[cfg(feature = "ap")]
    RaceEndDelay,
    #[cfg(feature = "ap")]
    RaceFinishCollapse,
    #[cfg(feature = "ap")]
    RaceFinishKill,
    #[cfg(feature = "ap")]
    RaceIdleKill,
    #[cfg(feature = "ap")]
    RaceIdleSpeed,
    #[cfg(feature = "ap")]
    RaceIdleTime,
    #[cfg(feature = "ap")]
    RaceIdleWarnings,
    #[cfg(feature = "ap")]
    RaceLaps,
    #[cfg(feature = "ap")]
    RaceLogLogin,
    #[cfg(feature = "ap")]
    RaceLogTime,
    #[cfg(feature = "ap")]
    RaceLogUnfinished,
    #[cfg(feature = "ap")]
    RaceNumRanksShowEnd,
    #[cfg(feature = "ap")]
    RaceNumRanksShowStart,
    #[cfg(feature = "ap")]
    RacePointsType,
    #[cfg(feature = "ap")]
    RaceRanksShowEnd,
    #[cfg(feature = "ap")]
    RaceRanksShowStart,
    #[cfg(feature = "ap")]
    RaceRankHeaderLength,
    #[cfg(feature = "ap")]
    RaceRankHeaderOrder,
    #[cfg(feature = "ap")]
    RaceRankHeaderPlayerLength,
    #[cfg(feature = "ap")]
    RaceRankHeaderPlayerOrder,
    #[cfg(feature = "ap")]
    RaceRankHeaderTimeLength,
    #[cfg(feature = "ap")]
    RaceRankHeaderTimeOrder,
    #[cfg(feature = "ap")]
    RaceRankShowLength,
    #[cfg(feature = "ap")]
    RaceRankShowPlayerLength,
    #[cfg(feature = "ap")]
    RaceRecordsLoad,
    #[cfg(feature = "ap")]
    RaceRecordsSave,
    #[cfg(feature = "ap")]
    RaceSafeAngles,
    #[cfg(feature = "ap")]
    RaceScoreDeplete,
    #[cfg(feature = "ap")]
    RaceSmartTimer,
    #[cfg(feature = "ap")]
    RaceSmartTimerFactor,
    #[cfg(feature = "ap")]
    RaceSmartTimerNum,
    #[cfg(feature = "ap")]
    RaceTimerEnabled,
    #[cfg(feature = "ap")]
    RaceUnsafeAnglesKill,
    #[cfg(feature = "ap")]
    ReloadConfig,
    #[cfg(feature = "ap")]
    ResetConfigQueueing,
    #[cfg(feature = "ap")]
    ResetMapQueueing,
    #[cfg(feature = "ap")]
    ResetRotation,
    #[cfg(feature = "ap")]
    ResetRotationOnStartNewMatch,
    #[cfg(feature = "ap")]
    Respawn,
    #[cfg(feature = "ap")]
    RespawnAll,
    #[cfg(feature = "ap")]
    RespawnDefaultPosition,
    #[cfg(feature = "ap")]
    RespawnMessage,
    #[cfg(feature = "ap")]
    RespawnPlayer,
    #[cfg(feature = "ap")]
    RespawnScript,
    #[cfg(feature = "ap")]
    RespawnStrict,
    #[cfg(feature = "ap")]
    RespawnTime,
    #[cfg(feature = "ap")]
    RevertMapFile,
    #[cfg(feature = "ap")]
    RotationMax,
    #[cfg(feature = "ap")]
    RotationMaxType,
    #[cfg(feature = "ap")]
    RotationMessage,
    #[cfg(feature = "ap")]
    RotationType,
    #[cfg(feature = "ap")]
    RubberzoneRate,
    #[cfg(feature = "ap")]
    ScoreBlastzone,
    #[cfg(feature = "ap")]
    ScoreDeathzoneTeam,
    #[cfg(feature = "ap")]
    ScoreDeathShot,
    #[cfg(feature = "ap")]
    ScoreDiffWin,
    #[cfg(feature = "ap")]
    ScoreExplosion,
    #[cfg(feature = "ap")]
    ScoreExplosionOwner,
    #[cfg(feature = "ap")]
    ScoreFlag,
    #[cfg(feature = "ap")]
    ScoreFlagHomeBase,
    #[cfg(feature = "ap")]
    ScoreGoal,
    #[cfg(feature = "ap")]
    ScoreRace,
    #[cfg(feature = "ap")]
    ScoreRaceFinish,
    #[cfg(feature = "ap")]
    ScoreRubberzone,
    #[cfg(feature = "ap")]
    ScoreSelfDestruct,
    #[cfg(feature = "ap")]
    ScoreShot,
    #[cfg(feature = "ap")]
    ScoreShotBase,
    #[cfg(feature = "ap")]
    ScoreShotSuicide,
    #[cfg(feature = "ap")]
    ScoreZombieZone,
    #[cfg(feature = "ap")]
    ScoreZombieZoneRevenge,
    #[cfg(feature = "ap")]
    ScriptEnv,
    #[cfg(feature = "ap")]
    SelfDestruct,
    #[cfg(feature = "ap")]
    SelfDestructFall,
    #[cfg(feature = "ap")]
    SelfDestructRadius,
    #[cfg(feature = "ap")]
    SelfDestructRise,
    #[cfg(feature = "ap")]
    SelfDestructRot,
    #[cfg(feature = "ap")]
    SelfDestructVanish,
    #[cfg(feature = "ap")]
    ServerOptions,
    #[cfg(feature = "ap")]
    SetAiPosition,
    #[cfg(feature = "ap")]
    SetCommandsAccesslevel,
    #[cfg(feature = "ap")]
    SetCycleBraking,
    #[cfg(feature = "ap")]
    SetCycleRubber,
    #[cfg(feature = "ap")]
    SetCycleSpeed,
    #[cfg(feature = "ap")]
    SetPlayerTeam,
    #[cfg(feature = "ap")]
    SetTargetCommand,
    #[cfg(feature = "ap")]
    SetZoneColor,
    #[cfg(feature = "ap")]
    SetZoneCoord,
    #[cfg(feature = "ap")]
    SetZoneDir,
    #[cfg(feature = "ap")]
    SetZoneExpansion,
    #[cfg(feature = "ap")]
    SetZoneIdColor,
    #[cfg(feature = "ap")]
    SetZoneIdCoord,
    #[cfg(feature = "ap")]
    SetZoneIdDir,
    #[cfg(feature = "ap")]
    SetZoneIdExpansion,
    #[cfg(feature = "ap")]
    SetZoneIdPenetrate,
    #[cfg(feature = "ap")]
    SetZoneIdRadius,
    #[cfg(feature = "ap")]
    SetZoneIdRotation,
    #[cfg(feature = "ap")]
    SetZoneIdRoute,
    #[cfg(feature = "ap")]
    SetZoneIdSpeed,
    #[cfg(feature = "ap")]
    SetZonePenetrate,
    #[cfg(feature = "ap")]
    SetZoneRadius,
    #[cfg(feature = "ap")]
    SetZoneRotation,
    #[cfg(feature = "ap")]
    SetZoneRoute,
    #[cfg(feature = "ap")]
    SetZoneSpeed,
    #[cfg(feature = "ap")]
    ShotBaseEnemyRespawn,
    #[cfg(feature = "ap")]
    ShotBaseRespawn,
    #[cfg(feature = "ap")]
    ShotCollision,
    #[cfg(feature = "ap")]
    ShotDiscardTime,
    #[cfg(feature = "ap")]
    ShotExplosion,
    #[cfg(feature = "ap")]
    ShotKillEnemies,
    #[cfg(feature = "ap")]
    ShotKillInvulnerable,
    #[cfg(feature = "ap")]
    ShotKillSelf,
    #[cfg(feature = "ap")]
    ShotKillVanish,
    #[cfg(feature = "ap")]
    ShotPenetrateWalls,
    #[cfg(feature = "ap")]
    ShotRadiusMax,
    #[cfg(feature = "ap")]
    ShotRadiusMin,
    #[cfg(feature = "ap")]
    ShotRotMax,
    #[cfg(feature = "ap")]
    ShotRotMin,
    #[cfg(feature = "ap")]
    ShotSeekUpdateTime,
    #[cfg(feature = "ap")]
    ShotStartDist,
    #[cfg(feature = "ap")]
    ShotThresh,
    #[cfg(feature = "ap")]
    ShotVelocityMult,
    #[cfg(feature = "ap")]
    ShotWallBounce,
    #[cfg(feature = "ap")]
    ShowMapAxes,
    #[cfg(feature = "ap")]
    ShowMapCreation,
    #[cfg(feature = "ap")]
    ShowMapDetails,
    #[cfg(feature = "ap")]
    ShufflePlayer,
    #[cfg(feature = "ap")]
    Shutdown,
    #[cfg(feature = "ap")]
    ShutdownStop,
    #[cfg(feature = "ap")]
    ShutdownTimeout,
    #[cfg(feature = "ap")]
    SilenceAll,
    #[cfg(feature = "ap")]
    SilenceDead,
    #[cfg(feature = "ap")]
    SilenceEnemies,
    #[cfg(feature = "ap")]
    SoccerBallFirstWin,
    #[cfg(feature = "ap")]
    SoccerBallScoreOwnGoal,
    #[cfg(feature = "ap")]
    SoccerBallShotsWin,
    #[cfg(feature = "ap")]
    SoccerBallSlowdown,
    #[cfg(feature = "ap")]
    SoccerBallSlowdownHackymethod,
    #[cfg(feature = "ap")]
    SoccerBallSlowdownSpeed,
    #[cfg(feature = "ap")]
    SoccerBallSlowdownSyncInterval,
    #[cfg(feature = "ap")]
    SoccerGoalKillEnemies,
    #[cfg(feature = "ap")]
    SoccerGoalRespawnAllies,
    #[cfg(feature = "ap")]
    SoccerGoalRespawnEnemies,
    #[cfg(feature = "ap")]
    SoccerGoalScore,
    #[cfg(feature = "ap")]
    SpawnAlternate,
    #[cfg(feature = "ap")]
    SpawnEnabled,
    #[cfg(feature = "ap")]
    SpawnExplosion,
    #[cfg(feature = "ap")]
    SpawnObjectzone,
    #[cfg(feature = "ap")]
    SpawnScript,
    #[cfg(feature = "ap")]
    SpawnSoccer,
    #[cfg(feature = "ap")]
    SpawnWinnersFirst,
    #[cfg(feature = "ap")]
    SpawnWrap,
    #[cfg(feature = "ap")]
    SpawnZone,
    #[cfg(feature = "ap")]
    SpeakAsAdmin,
    #[cfg(feature = "ap")]
    SpeakToEnemies,
    #[cfg(feature = "ap")]
    SpeakToEveryone,
    #[cfg(feature = "ap")]
    SpLimitAdvance,
    #[cfg(feature = "ap")]
    SpLimitScoreMinLead,
    #[cfg(feature = "ap")]
    SpLimitSets,
    #[cfg(feature = "ap")]
    SpScoreDiffWin,
    #[cfg(feature = "ap")]
    StyctCompatibilityLadderlogPlayerGridpos,
    #[cfg(feature = "ap")]
    StyctCompatibilitySetZoneColor,
    #[cfg(feature = "ap")]
    StyctCompatibilitySpawnZone,
    #[cfg(feature = "ap")]
    SuicideMessage,
    #[cfg(feature = "ap")]
    SuspendAll,
    #[cfg(feature = "ap")]
    SuspendList,
    #[cfg(feature = "ap")]
    SvgZoneRotationAnimate,
    #[cfg(feature = "ap")]
    SwapWinzoneDeathzoneColors,
    #[cfg(feature = "ap")]
    TakePoints,
    #[cfg(feature = "ap")]
    TargetzoneColorB,
    #[cfg(feature = "ap")]
    TargetzoneColorG,
    #[cfg(feature = "ap")]
    TargetzoneColorR,
    #[cfg(feature = "ap")]
    TargetDeclareWinner,
    #[cfg(feature = "ap")]
    TargetInitialScore,
    #[cfg(feature = "ap")]
    TargetLifetime,
    #[cfg(feature = "ap")]
    TargetPlayerMultiuse,
    #[cfg(feature = "ap")]
    TargetScoreDeplete,
    #[cfg(feature = "ap")]
    TargetSurviveTime,
    #[cfg(feature = "ap")]
    Teleport,
    #[cfg(feature = "ap")]
    TeleportPlayer,
    #[cfg(feature = "ap")]
    TextBrightBackground,
    #[cfg(feature = "ap")]
    TextShadow,
    #[cfg(feature = "ap")]
    TimerMax,
    #[cfg(feature = "ap")]
    TimerMin,
    #[cfg(feature = "ap")]
    TimerMode,
    #[cfg(feature = "ap")]
    TimerReset,
    #[cfg(feature = "ap")]
    TimerResume,
    #[cfg(feature = "ap")]
    TimerStart,
    #[cfg(feature = "ap")]
    TimerStop,
    #[cfg(feature = "ap")]
    TimerType,
    #[cfg(feature = "ap")]
    UnsilenceAll,
    #[cfg(feature = "ap")]
    UnsuspendAll,
    #[cfg(feature = "ap")]
    VerifyColorStrict,
    #[cfg(feature = "ap")]
    VoiceAll,
    #[cfg(feature = "ap")]
    VotingBiasChallenge,
    #[cfg(feature = "ap")]
    WinzonePlayerEnterWin,
    #[cfg(feature = "ap")]
    ZombieZone,
    #[cfg(feature = "ap")]
    ZombieZoneFall,
    #[cfg(feature = "ap")]
    ZombieZoneRadius,
    #[cfg(feature = "ap")]
    ZombieZoneRise,
    #[cfg(feature = "ap")]
    ZombieZoneRot,
    #[cfg(feature = "ap")]
    ZombieZoneShoot,
    #[cfg(feature = "ap")]
    ZombieZoneSpeed,
    #[cfg(feature = "ap")]
    ZombieZoneVanish,
    #[cfg(feature = "ap")]
    ZonesBounceOnCycleWalls,
    #[cfg(feature = "ap")]
    ZoneDelayClear,
    #[cfg(feature = "ap")]
    ZoneGridposInterval,
    #[cfg(feature = "ap")]
    ZoneHeightFort,
    #[cfg(feature = "ap")]
    ZoneHeightKoh,
    #[cfg(feature = "ap")]
    ZoneNoFadeInServer,
    #[cfg(feature = "ap")]
    ZoneSegSteps,
    #[cfg(feature = "ap")]
    ZoneSpeedDecay,
    #[cfg(feature = "ap")]
    ZoneSpeedHitDecay,
    #[cfg(feature = "ap")]
    ZoneWallBoundary,
    #[cfg(feature = "ap")]
    ZoneWallBoundaryValueRestricted,
    #[cfg(feature = "ap")]
    ZoneWallDeath,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let word = match self {
            Command::AccessLevel => "ACCESS_LEVEL",
            Command::AccessLevelAnnounceLogin => "ACCESS_LEVEL_ANNOUNCE_LOGIN",
            Command::AccessLevelAutokickImmunity => "ACCESS_LEVEL_AUTOKICK_IMMUNITY",
            Command::AccessLevelChat => "ACCESS_LEVEL_CHAT",
            Command::AccessLevelChatTimeout => "ACCESS_LEVEL_CHAT_TIMEOUT",
            Command::AccessLevelHideOf => "ACCESS_LEVEL_HIDE_OF",
            Command::AccessLevelHideTo => "ACCESS_LEVEL_HIDE_TO",
            Command::AccessLevelIps => "ACCESS_LEVEL_IPS",
            Command::AccessLevelListAdmins => "ACCESS_LEVEL_LIST_ADMINS",
            Command::AccessLevelListAdminsSeeEveryone => "ACCESS_LEVEL_LIST_ADMINS_SEE_EVERYONE",
            Command::AccessLevelNver => "ACCESS_LEVEL_NVER",
            Command::AccessLevelPlay => "ACCESS_LEVEL_PLAY",
            Command::AccessLevelPlaySliders => "ACCESS_LEVEL_PLAY_SLIDERS",
            Command::AccessLevelPlaySliding => "ACCESS_LEVEL_PLAY_SLIDING",
            Command::AccessLevelRtfm => "ACCESS_LEVEL_RTFM",
            Command::AccessLevelShout => "ACCESS_LEVEL_SHOUT",
            Command::AccessLevelShuffleUp => "ACCESS_LEVEL_SHUFFLE_UP",
            Command::AccessLevelSpyMsg => "ACCESS_LEVEL_SPY_MSG",
            Command::AccessLevelSpyTeam => "ACCESS_LEVEL_SPY_TEAM",
            Command::AccessLevelVoteCommand => "ACCESS_LEVEL_VOTE_COMMAND",
            Command::AccessLevelVoteCommandExecute => "ACCESS_LEVEL_VOTE_COMMAND_EXECUTE",
            Command::AccessLevelVoteInclude => "ACCESS_LEVEL_VOTE_INCLUDE",
            Command::AccessLevelVoteIncludeExecute => "ACCESS_LEVEL_VOTE_INCLUDE_EXECUTE",
            Command::AccessLevelVoteKick => "ACCESS_LEVEL_VOTE_KICK",
            Command::AccessLevelVoteSilence => "ACCESS_LEVEL_VOTE_SILENCE",
            Command::AccessLevelVoteSuspend => "ACCESS_LEVEL_VOTE_SUSPEND",
            Command::AddHelpTopic => "ADD_HELP_TOPIC",
            Command::AddMasterServer => "ADD_MASTER_SERVER",
            Command::Admins => "ADMINS",
            Command::AdminListColorsBestBlue => "ADMIN_LIST_COLORS_BEST_BLUE",
            Command::AdminListColorsBestGreen => "ADMIN_LIST_COLORS_BEST_GREEN",
            Command::AdminListColorsBestRed => "ADMIN_LIST_COLORS_BEST_RED",
            Command::AdminListColorsWorstBlue => "ADMIN_LIST_COLORS_WORST_BLUE",
            Command::AdminListColorsWorstGreen => "ADMIN_LIST_COLORS_WORST_GREEN",
            Command::AdminListColorsWorstRed => "ADMIN_LIST_COLORS_WORST_RED",
            Command::AdminListMinAccessLevel => "ADMIN_LIST_MIN_ACCESS_LEVEL",
            Command::AiIq => "AI_IQ",
            Command::AliveLocx => "ALIVE_LOCX",
            Command::AliveLocy => "ALIVE_LOCY",
            Command::AliveSize => "ALIVE_SIZE",
            Command::AllowCam1_0 => "ALLOW_CAM_1_0",
            Command::AllowCam1_1 => "ALLOW_CAM_1_1",
            Command::AllowCam1_2 => "ALLOW_CAM_1_2",
            Command::AllowCam1_3 => "ALLOW_CAM_1_3",
            Command::AllowCam1_4 => "ALLOW_CAM_1_4",
            Command::AllowCam1_5 => "ALLOW_CAM_1_5",
            Command::AllowCam1_6 => "ALLOW_CAM_1_6",
            Command::AllowCam2_0 => "ALLOW_CAM_2_0",
            Command::AllowCam2_1 => "ALLOW_CAM_2_1",
            Command::AllowCam2_2 => "ALLOW_CAM_2_2",
            Command::AllowCam2_3 => "ALLOW_CAM_2_3",
            Command::AllowCam2_4 => "ALLOW_CAM_2_4",
            Command::AllowCam2_5 => "ALLOW_CAM_2_5",
            Command::AllowCam2_6 => "ALLOW_CAM_2_6",
            Command::AllowCam3_0 => "ALLOW_CAM_3_0",
            Command::AllowCam3_1 => "ALLOW_CAM_3_1",
            Command::AllowCam3_2 => "ALLOW_CAM_3_2",
            Command::AllowCam3_3 => "ALLOW_CAM_3_3",
            Command::AllowCam3_4 => "ALLOW_CAM_3_4",
            Command::AllowCam3_5 => "ALLOW_CAM_3_5",
            Command::AllowCam3_6 => "ALLOW_CAM_3_6",
            Command::AllowCam4_0 => "ALLOW_CAM_4_0",
            Command::AllowCam4_1 => "ALLOW_CAM_4_1",
            Command::AllowCam4_2 => "ALLOW_CAM_4_2",
            Command::AllowCam4_3 => "ALLOW_CAM_4_3",
            Command::AllowCam4_4 => "ALLOW_CAM_4_4",
            Command::AllowCam4_5 => "ALLOW_CAM_4_5",
            Command::AllowCam4_6 => "ALLOW_CAM_4_6",
            Command::AllowControlDuringChat => "ALLOW_CONTROL_DURING_CHAT",
            Command::AllowEnemiesSameClient => "ALLOW_ENEMIES_SAME_CLIENT",
            Command::AllowEnemiesSameIp => "ALLOW_ENEMIES_SAME_IP",
            Command::AllowImposters => "ALLOW_IMPOSTERS",
            Command::AllowImpostors => "ALLOW_IMPOSTORS",
            Command::AllowRenamePlayer => "ALLOW_RENAME_PLAYER",
            Command::AllowTeamChange => "ALLOW_TEAM_CHANGE",
            Command::AllowTeamChangePlayer => "ALLOW_TEAM_CHANGE_PLAYER",
            Command::AllowTeamNameColor => "ALLOW_TEAM_NAME_COLOR",
            Command::AllowTeamNamePlayer => "ALLOW_TEAM_NAME_PLAYER",
            Command::AllowVoting => "ALLOW_VOTING",
            Command::AllowVotingSpectator => "ALLOW_VOTING_SPECTATOR",
            Command::AlphaBlend => "ALPHA_BLEND",
            Command::AntiSpoof => "ANTI_SPOOF",
            Command::ArenaAxes => "ARENA_AXES",
            Command::ArenaAxesOverride => "ARENA_AXES_OVERRIDE",
            Command::ArmagetronLastScreenmode => "ARMAGETRON_LAST_SCREENMODE",
            Command::ArmagetronLastWindowsize => "ARMAGETRON_LAST_WINDOWSIZE",
            Command::ArmagetronScreenmode => "ARMAGETRON_SCREENMODE",
            Command::ArmagetronVsync => "ARMAGETRON_VSYNC",
            Command::ArmagetronVsyncLast => "ARMAGETRON_VSYNC_LAST",
            Command::ArmagetronWindowsize => "ARMAGETRON_WINDOWSIZE",
            Command::AuthorityBlacklist => "AUTHORITY_BLACKLIST",
            Command::AuthorityLevel => "AUTHORITY_LEVEL",
            Command::AuthorityWhitelist => "AUTHORITY_WHITELIST",
            Command::AutoAis => "AUTO_AIS",
            Command::AutoIncam1 => "AUTO_INCAM_1",
            Command::AutoIncam2 => "AUTO_INCAM_2",
            Command::AutoIncam3 => "AUTO_INCAM_3",
            Command::AutoIncam4 => "AUTO_INCAM_4",
            Command::AutoIq => "AUTO_IQ",
            Command::AutoLogin1 => "AUTO_LOGIN_1",
            Command::AutoLogin2 => "AUTO_LOGIN_2",
            Command::AutoLogin3 => "AUTO_LOGIN_3",
            Command::AutoLogin4 => "AUTO_LOGIN_4",
            Command::AutoTeam => "AUTO_TEAM",
            Command::AutoTeamSpecSpam => "AUTO_TEAM_SPEC_SPAM",
            Command::AxesIndicator => "AXES_INDICATOR",
            Command::BackwardCompatibility => "BACKWARD_COMPATIBILITY",
            Command::Ban => "BAN",
            Command::BanIp => "BAN_IP",
            Command::BanList => "BAN_LIST",
            Command::BanUser => "BAN_USER",
            Command::BanUserList => "BAN_USER_LIST",
            Command::BigBrother => "BIG_BROTHER",
            Command::Bookmark0Address => "BOOKMARK_0_ADDRESS",
            Command::Bookmark0Name => "BOOKMARK_0_NAME",
            Command::Bookmark0Port => "BOOKMARK_0_PORT",
            Command::Bookmark10Address => "BOOKMARK_10_ADDRESS",
            Command::Bookmark10Name => "BOOKMARK_10_NAME",
            Command::Bookmark10Port => "BOOKMARK_10_PORT",
            Command::Bookmark1Address => "BOOKMARK_1_ADDRESS",
            Command::Bookmark1Name => "BOOKMARK_1_NAME",
            Command::Bookmark1Port => "BOOKMARK_1_PORT",
            Command::Bookmark2Address => "BOOKMARK_2_ADDRESS",
            Command::Bookmark2Name => "BOOKMARK_2_NAME",
            Command::Bookmark2Port => "BOOKMARK_2_PORT",
            Command::Bookmark3Address => "BOOKMARK_3_ADDRESS",
            Command::Bookmark3Name => "BOOKMARK_3_NAME",
            Command::Bookmark3Port => "BOOKMARK_3_PORT",
            Command::Bookmark4Address => "BOOKMARK_4_ADDRESS",
            Command::Bookmark4Name => "BOOKMARK_4_NAME",
            Command::Bookmark4Port => "BOOKMARK_4_PORT",
            Command::Bookmark5Address => "BOOKMARK_5_ADDRESS",
            Command::Bookmark5Name => "BOOKMARK_5_NAME",
            Command::Bookmark5Port => "BOOKMARK_5_PORT",
            Command::Bookmark6Address => "BOOKMARK_6_ADDRESS",
            Command::Bookmark6Name => "BOOKMARK_6_NAME",
            Command::Bookmark6Port => "BOOKMARK_6_PORT",
            Command::Bookmark7Address => "BOOKMARK_7_ADDRESS",
            Command::Bookmark7Name => "BOOKMARK_7_NAME",
            Command::Bookmark7Port => "BOOKMARK_7_PORT",
            Command::Bookmark8Address => "BOOKMARK_8_ADDRESS",
            Command::Bookmark8Name => "BOOKMARK_8_NAME",
            Command::Bookmark8Port => "BOOKMARK_8_PORT",
            Command::Bookmark9Address => "BOOKMARK_9_ADDRESS",
            Command::Bookmark9Name => "BOOKMARK_9_NAME",
            Command::Bookmark9Port => "BOOKMARK_9_PORT",
            Command::BookmarkMaster0Address => "BOOKMARK__MASTER0_ADDRESS",
            Command::BookmarkMaster0Name => "BOOKMARK__MASTER0_NAME",
            Command::BookmarkMaster0Port => "BOOKMARK__MASTER0_PORT",
            Command::BookmarkMaster10Address => "BOOKMARK__MASTER10_ADDRESS",
            Command::BookmarkMaster10Name => "BOOKMARK__MASTER10_NAME",
            Command::BookmarkMaster10Port => "BOOKMARK__MASTER10_PORT",
            Command::BookmarkMaster1Address => "BOOKMARK__MASTER1_ADDRESS",
            Command::BookmarkMaster1Name => "BOOKMARK__MASTER1_NAME",
            Command::BookmarkMaster1Port => "BOOKMARK__MASTER1_PORT",
            Command::BookmarkMaster2Address => "BOOKMARK__MASTER2_ADDRESS",
            Command::BookmarkMaster2Name => "BOOKMARK__MASTER2_NAME",
            Command::BookmarkMaster2Port => "BOOKMARK__MASTER2_PORT",
            Command::BookmarkMaster3Address => "BOOKMARK__MASTER3_ADDRESS",
            Command::BookmarkMaster3Name => "BOOKMARK__MASTER3_NAME",
            Command::BookmarkMaster3Port => "BOOKMARK__MASTER3_PORT",
            Command::BookmarkMaster4Address => "BOOKMARK__MASTER4_ADDRESS",
            Command::BookmarkMaster4Name => "BOOKMARK__MASTER4_NAME",
            Command::BookmarkMaster4Port => "BOOKMARK__MASTER4_PORT",
            Command::BookmarkMaster5Address => "BOOKMARK__MASTER5_ADDRESS",
            Command::BookmarkMaster5Name => "BOOKMARK__MASTER5_NAME",
            Command::BookmarkMaster5Port => "BOOKMARK__MASTER5_PORT",
            Command::BookmarkMaster6Address => "BOOKMARK__MASTER6_ADDRESS",
            Command::BookmarkMaster6Name => "BOOKMARK__MASTER6_NAME",
            Command::BookmarkMaster6Port => "BOOKMARK__MASTER6_PORT",
            Command::BookmarkMaster7Address => "BOOKMARK__MASTER7_ADDRESS",
            Command::BookmarkMaster7Name => "BOOKMARK__MASTER7_NAME",
            Command::BookmarkMaster7Port => "BOOKMARK__MASTER7_PORT",
            Command::BookmarkMaster8Address => "BOOKMARK__MASTER8_ADDRESS",
            Command::BookmarkMaster8Name => "BOOKMARK__MASTER8_NAME",
            Command::BookmarkMaster8Port => "BOOKMARK__MASTER8_PORT",
            Command::BookmarkMaster9Address => "BOOKMARK__MASTER9_ADDRESS",
            Command::BookmarkMaster9Name => "BOOKMARK__MASTER9_NAME",
            Command::BookmarkMaster9Port => "BOOKMARK__MASTER9_PORT",
            Command::BrakeGaugeLocx => "BRAKE_GAUGE_LOCX",
            Command::BrakeGaugeLocy => "BRAKE_GAUGE_LOCY",
            Command::BrakeGaugeSize => "BRAKE_GAUGE_SIZE",
            Command::BugColorOverflow => "BUG_COLOR_OVERFLOW",
            Command::BugRip => "BUG_RIP",
            Command::BugTransparency => "BUG_TRANSPARENCY",
            Command::BugTransparencyDemand => "BUG_TRANSPARENCY_DEMAND",
            Command::BugTunnel => "BUG_TUNNEL",
            Command::Camcenter1 => "CAMCENTER_1",
            Command::Camcenter2 => "CAMCENTER_2",
            Command::Camcenter3 => "CAMCENTER_3",
            Command::Camcenter4 => "CAMCENTER_4",
            Command::CameraCustomBack => "CAMERA_CUSTOM_BACK",
            Command::CameraCustomBackFromspeed => "CAMERA_CUSTOM_BACK_FROMSPEED",
            Command::CameraCustomPitch => "CAMERA_CUSTOM_PITCH",
            Command::CameraCustomRise => "CAMERA_CUSTOM_RISE",
            Command::CameraCustomRiseFromspeed => "CAMERA_CUSTOM_RISE_FROMSPEED",
            Command::CameraCustomTurnSpeed => "CAMERA_CUSTOM_TURN_SPEED",
            Command::CameraCustomTurnSpeed180 => "CAMERA_CUSTOM_TURN_SPEED_180",
            Command::CameraCustomZoom => "CAMERA_CUSTOM_ZOOM",
            Command::CameraFollowStartX => "CAMERA_FOLLOW_START_X",
            Command::CameraFollowStartY => "CAMERA_FOLLOW_START_Y",
            Command::CameraFollowStartZ => "CAMERA_FOLLOW_START_Z",
            Command::CameraForbidCustom => "CAMERA_FORBID_CUSTOM",
            Command::CameraForbidCustomGlance => "CAMERA_FORBID_CUSTOM_GLANCE",
            Command::CameraForbidFollow => "CAMERA_FORBID_FOLLOW",
            Command::CameraForbidFree => "CAMERA_FORBID_FREE",
            Command::CameraForbidIn => "CAMERA_FORBID_IN",
            Command::CameraForbidServerCustom => "CAMERA_FORBID_SERVER_CUSTOM",
            Command::CameraForbidSmart => "CAMERA_FORBID_SMART",
            Command::CameraFreeStartX => "CAMERA_FREE_START_X",
            Command::CameraFreeStartY => "CAMERA_FREE_START_Y",
            Command::CameraFreeStartZ => "CAMERA_FREE_START_Z",
            Command::CameraGlanceBack => "CAMERA_GLANCE_BACK",
            Command::CameraGlanceBackFromspeed => "CAMERA_GLANCE_BACK_FROMSPEED",
            Command::CameraGlancePitch => "CAMERA_GLANCE_PITCH",
            Command::CameraGlanceRise => "CAMERA_GLANCE_RISE",
            Command::CameraGlanceRiseFromspeed => "CAMERA_GLANCE_RISE_FROMSPEED",
            Command::CameraInTurnSpeed => "CAMERA_IN_TURN_SPEED",
            Command::CameraOverrideCustomGlance => "CAMERA_OVERRIDE_CUSTOM_GLANCE",
            Command::CameraOverrideCustomGlanceServerCustom => {
                "CAMERA_OVERRIDE_CUSTOM_GLANCE_SERVER_CUSTOM"
            }
            Command::CameraServerCustomBack => "CAMERA_SERVER_CUSTOM_BACK",
            Command::CameraServerCustomBackFromspeed => "CAMERA_SERVER_CUSTOM_BACK_FROMSPEED",
            Command::CameraServerCustomPitch => "CAMERA_SERVER_CUSTOM_PITCH",
            Command::CameraServerCustomRise => "CAMERA_SERVER_CUSTOM_RISE",
            Command::CameraServerCustomRiseFromspeed => "CAMERA_SERVER_CUSTOM_RISE_FROMSPEED",
            Command::CameraServerCustomTurnSpeed => "CAMERA_SERVER_CUSTOM_TURN_SPEED",
            Command::CameraServerCustomTurnSpeed180 => "CAMERA_SERVER_CUSTOM_TURN_SPEED_180",
            Command::CameraServerGlanceBack => "CAMERA_SERVER_GLANCE_BACK",
            Command::CameraServerGlanceBackFromspeed => "CAMERA_SERVER_GLANCE_BACK_FROMSPEED",
            Command::CameraServerGlancePitch => "CAMERA_SERVER_GLANCE_PITCH",
            Command::CameraServerGlanceRise => "CAMERA_SERVER_GLANCE_RISE",
            Command::CameraServerGlanceRiseFromspeed => "CAMERA_SERVER_GLANCE_RISE_FROMSPEED",
            Command::CameraSmartStartX => "CAMERA_SMART_START_X",
            Command::CameraSmartStartY => "CAMERA_SMART_START_Y",
            Command::CameraSmartStartZ => "CAMERA_SMART_START_Z",
            Command::CameraVisibilityClipSpeed => "CAMERA_VISIBILITY_CLIP_SPEED",
            Command::CameraVisibilityExtension => "CAMERA_VISIBILITY_EXTENSION",
            Command::CameraVisibilityLowerWall => "CAMERA_VISIBILITY_LOWER_WALL",
            Command::CameraVisibilityLowerWallSmart => "CAMERA_VISIBILITY_LOWER_WALL_SMART",
            Command::CameraVisibilityRecoverySpeed => "CAMERA_VISIBILITY_RECOVERY_SPEED",
            Command::CameraVisibilitySideskew => "CAMERA_VISIBILITY_SIDESKEW",
            Command::CameraVisibilityWallDistance => "CAMERA_VISIBILITY_WALL_DISTANCE",
            Command::Camwobble1 => "CAMWOBBLE_1",
            Command::Camwobble2 => "CAMWOBBLE_2",
            Command::Camwobble3 => "CAMWOBBLE_3",
            Command::Camwobble4 => "CAMWOBBLE_4",
            Command::Casacl => "CASACL",
            Command::CenterMessage => "CENTER_MESSAGE",
            Command::ChatbotAlwaysActive => "CHATBOT_ALWAYS_ACTIVE",
            Command::ChatbotDecay => "CHATBOT_DECAY",
            Command::ChatbotDelay => "CHATBOT_DELAY",
            Command::ChatbotMinTimestep => "CHATBOT_MIN_TIMESTEP",
            Command::ChatbotNewWallBlindness => "CHATBOT_NEW_WALL_BLINDNESS",
            Command::ChatbotRange => "CHATBOT_RANGE",
            Command::ChatterRemoveTime => "CHATTER_REMOVE_TIME",
            Command::ChatLog => "CHAT_LOG",
            Command::ChatTooltip => "CHAT_TOOLTIP",
            Command::CheckErrors => "CHECK_ERRORS",
            Command::ClientPort => "CLIENT_PORT",
            Command::CmLocy => "CM_LOCY",
            Command::Colordepth => "COLORDEPTH",
            Command::ColorB1 => "COLOR_B_1",
            Command::ColorB2 => "COLOR_B_2",
            Command::ColorB3 => "COLOR_B_3",
            Command::ColorB4 => "COLOR_B_4",
            Command::ColorG1 => "COLOR_G_1",
            Command::ColorG2 => "COLOR_G_2",
            Command::ColorG3 => "COLOR_G_3",
            Command::ColorG4 => "COLOR_G_4",
            Command::ColorR1 => "COLOR_R_1",
            Command::ColorR2 => "COLOR_R_2",
            Command::ColorR3 => "COLOR_R_3",
            Command::ColorR4 => "COLOR_R_4",
            Command::ColorStrings => "COLOR_STRINGS",
            Command::ConnectionFloodSensitivity => "CONNECTION_FLOOD_SENSITIVITY",
            Command::ConnectionLimit => "CONNECTION_LIMIT",
            Command::ConsoleColumns => "CONSOLE_COLUMNS",
            Command::ConsoleIndent => "CONSOLE_INDENT",
            Command::ConsoleLadderLog => "CONSOLE_LADDER_LOG",
            Command::ConsoleLog => "CONSOLE_LOG",
            Command::ConsoleMessage => "CONSOLE_MESSAGE",
            Command::ConsoleRows => "CONSOLE_ROWS",
            Command::ConsoleRowsMax => "CONSOLE_ROWS_MAX",
            Command::CustomScreenAspect => "CUSTOM_SCREEN_ASPECT",
            Command::CustomScreenHeight => "CUSTOM_SCREEN_HEIGHT",
            Command::CustomScreenWidth => "CUSTOM_SCREEN_WIDTH",
            Command::CustomServerName => "CUSTOM_SERVER_NAME",
            Command::CycleAccel => "CYCLE_ACCEL",
            Command::CycleAccelEnemy => "CYCLE_ACCEL_ENEMY",
            Command::CycleAccelEnemyOverride => "CYCLE_ACCEL_ENEMY_OVERRIDE",
            Command::CycleAccelOffset => "CYCLE_ACCEL_OFFSET",
            Command::CycleAccelRim => "CYCLE_ACCEL_RIM",
            Command::CycleAccelRimOverride => "CYCLE_ACCEL_RIM_OVERRIDE",
            Command::CycleAccelSelf => "CYCLE_ACCEL_SELF",
            Command::CycleAccelSelfOverride => "CYCLE_ACCEL_SELF_OVERRIDE",
            Command::CycleAccelSlingshot => "CYCLE_ACCEL_SLINGSHOT",
            Command::CycleAccelSlingshotOverride => "CYCLE_ACCEL_SLINGSHOT_OVERRIDE",
            Command::CycleAccelTeam => "CYCLE_ACCEL_TEAM",
            Command::CycleAccelTeamOverride => "CYCLE_ACCEL_TEAM_OVERRIDE",
            Command::CycleAccelTunnel => "CYCLE_ACCEL_TUNNEL",
            Command::CycleAccelTunnelOverride => "CYCLE_ACCEL_TUNNEL_OVERRIDE",
            Command::CycleAvoidOldclientBadSync => "CYCLE_AVOID_OLDCLIENT_BAD_SYNC",
            Command::CycleBlinkFrequency => "CYCLE_BLINK_FREQUENCY",
            Command::CycleBoostfactorEnemy => "CYCLE_BOOSTFACTOR_ENEMY",
            Command::CycleBoostfactorEnemyOverride => "CYCLE_BOOSTFACTOR_ENEMY_OVERRIDE",
            Command::CycleBoostfactorRim => "CYCLE_BOOSTFACTOR_RIM",
            Command::CycleBoostfactorRimOverride => "CYCLE_BOOSTFACTOR_RIM_OVERRIDE",
            Command::CycleBoostfactorSelf => "CYCLE_BOOSTFACTOR_SELF",
            Command::CycleBoostfactorSelfOverride => "CYCLE_BOOSTFACTOR_SELF_OVERRIDE",
            Command::CycleBoostfactorTeam => "CYCLE_BOOSTFACTOR_TEAM",
            Command::CycleBoostfactorTeamOverride => "CYCLE_BOOSTFACTOR_TEAM_OVERRIDE",
            Command::CycleBoostEnemy => "CYCLE_BOOST_ENEMY",
            Command::CycleBoostEnemyOverride => "CYCLE_BOOST_ENEMY_OVERRIDE",
            Command::CycleBoostRim => "CYCLE_BOOST_RIM",
            Command::CycleBoostRimOverride => "CYCLE_BOOST_RIM_OVERRIDE",
            Command::CycleBoostSelf => "CYCLE_BOOST_SELF",
            Command::CycleBoostSelfOverride => "CYCLE_BOOST_SELF_OVERRIDE",
            Command::CycleBoostTeam => "CYCLE_BOOST_TEAM",
            Command::CycleBoostTeamOverride => "CYCLE_BOOST_TEAM_OVERRIDE",
            Command::CycleBrake => "CYCLE_BRAKE",
            Command::CycleBrakeDeplete => "CYCLE_BRAKE_DEPLETE",
            Command::CycleBrakeDepleteOverride => "CYCLE_BRAKE_DEPLETE_OVERRIDE",
            Command::CycleBrakeRefill => "CYCLE_BRAKE_REFILL",
            Command::CycleBrakeRefillOverride => "CYCLE_BRAKE_REFILL_OVERRIDE",
            Command::CycleBrakeTooltip => "CYCLE_BRAKE_TOOLTIP",
            Command::CycleDelay => "CYCLE_DELAY",
            Command::CycleDelayDoublebindBonus => "CYCLE_DELAY_DOUBLEBIND_BONUS",
            Command::CycleDelayDoublebindBonusOverride => "CYCLE_DELAY_DOUBLEBIND_BONUS_OVERRIDE",
            Command::CycleDelayTimebased => "CYCLE_DELAY_TIMEBASED",
            Command::CycleDelayTimebasedOverride => "CYCLE_DELAY_TIMEBASED_OVERRIDE",
            Command::CycleDistWallShrink => "CYCLE_DIST_WALL_SHRINK",
            Command::CycleDistWallShrinkOffset => "CYCLE_DIST_WALL_SHRINK_OFFSET",
            Command::CycleFairAntilag => "CYCLE_FAIR_ANTILAG",
            Command::CycleFirstSpawnProtection => "CYCLE_FIRST_SPAWN_PROTECTION",
            Command::CycleInvulnerableTime => "CYCLE_INVULNERABLE_TIME",
            Command::CycleMaxRefcount => "CYCLE_MAX_REFCOUNT",
            Command::CyclePacketlossTolerance => "CYCLE_PACKETLOSS_TOLERANCE",
            Command::CyclePingRubber => "CYCLE_PING_RUBBER",
            Command::CycleRubber => "CYCLE_RUBBER",
            Command::CycleRubberDelay => "CYCLE_RUBBER_DELAY",
            Command::CycleRubberDelayBonus => "CYCLE_RUBBER_DELAY_BONUS",
            Command::CycleRubberDelayBonusOverride => "CYCLE_RUBBER_DELAY_BONUS_OVERRIDE",
            Command::CycleRubberDelayOverride => "CYCLE_RUBBER_DELAY_OVERRIDE",
            Command::CycleRubberLegacy => "CYCLE_RUBBER_LEGACY",
            Command::CycleRubberMalusTurnOverride => "CYCLE_RUBBER_MALUS_TURN_OVERRIDE",
            Command::CycleRubberMinadjust => "CYCLE_RUBBER_MINADJUST",
            Command::CycleRubberMinadjustOverride => "CYCLE_RUBBER_MINADJUST_OVERRIDE",
            Command::CycleRubberMindistance => "CYCLE_RUBBER_MINDISTANCE",
            Command::CycleRubberMindistanceGap => "CYCLE_RUBBER_MINDISTANCE_GAP",
            Command::CycleRubberMindistanceGapBackdoor => "CYCLE_RUBBER_MINDISTANCE_GAP_BACKDOOR",
            Command::CycleRubberMindistanceGapBackdoorOverride => {
                "CYCLE_RUBBER_MINDISTANCE_GAP_BACKDOOR_OVERRIDE"
            }
            Command::CycleRubberMindistanceGapOverride => "CYCLE_RUBBER_MINDISTANCE_GAP_OVERRIDE",
            Command::CycleRubberMindistanceGapSide => "CYCLE_RUBBER_MINDISTANCE_GAP_SIDE",
            Command::CycleRubberMindistanceLegacy => "CYCLE_RUBBER_MINDISTANCE_LEGACY",
            Command::CycleRubberMindistanceOverride => "CYCLE_RUBBER_MINDISTANCE_OVERRIDE",
            Command::CycleRubberMindistancePreparation => "CYCLE_RUBBER_MINDISTANCE_PREPARATION",
            Command::CycleRubberMindistancePreparationOverride => {
                "CYCLE_RUBBER_MINDISTANCE_PREPARATION_OVERRIDE"
            }
            Command::CycleRubberMindistanceRatio => "CYCLE_RUBBER_MINDISTANCE_RATIO",
            Command::CycleRubberMindistanceRatioOverride => {
                "CYCLE_RUBBER_MINDISTANCE_RATIO_OVERRIDE"
            }
            Command::CycleRubberMindistanceReservoir => "CYCLE_RUBBER_MINDISTANCE_RESERVOIR",
            Command::CycleRubberMindistanceReservoirOverride => {
                "CYCLE_RUBBER_MINDISTANCE_RESERVOIR_OVERRIDE"
            }
            Command::CycleRubberMindistanceUnprepared => "CYCLE_RUBBER_MINDISTANCE_UNPREPARED",
            Command::CycleRubberMindistanceUnpreparedOverride => {
                "CYCLE_RUBBER_MINDISTANCE_UNPREPARED_OVERRIDE"
            }
            Command::CycleRubberSpeed => "CYCLE_RUBBER_SPEED",
            Command::CycleRubberSpeedOverride => "CYCLE_RUBBER_SPEED_OVERRIDE",
            Command::CycleRubberTime => "CYCLE_RUBBER_TIME",
            Command::CycleRubberTimebased => "CYCLE_RUBBER_TIMEBASED",
            Command::CycleRubberTimebasedOverride => "CYCLE_RUBBER_TIMEBASED_OVERRIDE",
            Command::CycleRubberTimeOverride => "CYCLE_RUBBER_TIME_OVERRIDE",
            Command::CycleRubberWallShrink => "CYCLE_RUBBER_WALL_SHRINK",
            Command::CycleRubberWallShrinkOverride => "CYCLE_RUBBER_WALL_SHRINK_OVERRIDE",
            Command::CycleSmoothMinSpeed => "CYCLE_SMOOTH_MIN_SPEED",
            Command::CycleSmoothThreshold => "CYCLE_SMOOTH_THRESHOLD",
            Command::CycleSmoothTime => "CYCLE_SMOOTH_TIME",
            Command::CycleSoundSpeed => "CYCLE_SOUND_SPEED",
            Command::CycleSpeed => "CYCLE_SPEED",
            Command::CycleSpeedDecayAbove => "CYCLE_SPEED_DECAY_ABOVE",
            Command::CycleSpeedDecayAboveOverride => "CYCLE_SPEED_DECAY_ABOVE_OVERRIDE",
            Command::CycleSpeedDecayBelow => "CYCLE_SPEED_DECAY_BELOW",
            Command::CycleSpeedDecayBelowOverride => "CYCLE_SPEED_DECAY_BELOW_OVERRIDE",
            Command::CycleSpeedMax => "CYCLE_SPEED_MAX",
            Command::CycleSpeedMaxOverride => "CYCLE_SPEED_MAX_OVERRIDE",
            Command::CycleSpeedMin => "CYCLE_SPEED_MIN",
            Command::CycleSpeedMinOverride => "CYCLE_SPEED_MIN_OVERRIDE",
            Command::CycleStartSpeed => "CYCLE_START_SPEED",
            Command::CycleSyncFf => "CYCLE_SYNC_FF",
            Command::CycleSyncFfSteps => "CYCLE_SYNC_FF_STEPS",
            Command::CycleSyncIntervalEnemy => "CYCLE_SYNC_INTERVAL_ENEMY",
            Command::CycleSyncIntervalSelf => "CYCLE_SYNC_INTERVAL_SELF",
            Command::CycleTimeTolerance => "CYCLE_TIME_TOLERANCE",
            Command::CycleTimeToleranceOverride => "CYCLE_TIME_TOLERANCE_OVERRIDE",
            Command::CycleTurnLeftTooltip => "CYCLE_TURN_LEFT_TOOLTIP",
            Command::CycleTurnMemory => "CYCLE_TURN_MEMORY",
            Command::CycleTurnRightTooltip => "CYCLE_TURN_RIGHT_TOOLTIP",
            Command::CycleTurnSpeedFactor => "CYCLE_TURN_SPEED_FACTOR",
            Command::CycleTurnSpeedFactorOverride => "CYCLE_TURN_SPEED_FACTOR_OVERRIDE",
            Command::CycleWallNear => "CYCLE_WALL_NEAR",
            Command::CycleWallTime => "CYCLE_WALL_TIME",
            Command::CycleWidth => "CYCLE_WIDTH",
            Command::CycleWidthOverride => "CYCLE_WIDTH_OVERRIDE",
            Command::CycleWidthRubberMax => "CYCLE_WIDTH_RUBBER_MAX",
            Command::CycleWidthRubberMaxOverride => "CYCLE_WIDTH_RUBBER_MAX_OVERRIDE",
            Command::CycleWidthRubberMin => "CYCLE_WIDTH_RUBBER_MIN",
            Command::CycleWidthRubberMinOverride => "CYCLE_WIDTH_RUBBER_MIN_OVERRIDE",
            Command::CycleWidthSide => "CYCLE_WIDTH_SIDE",
            Command::CycleWidthSideOverride => "CYCLE_WIDTH_SIDE_OVERRIDE",
            Command::DeclareRoundWinner => "DECLARE_ROUND_WINNER",
            Command::DedicatedIdle => "DEDICATED_IDLE",
            Command::DefaultKickReason => "DEFAULT_KICK_REASON",
            Command::DefaultKickToPort => "DEFAULT_KICK_TO_PORT",
            Command::DefaultKickToReason => "DEFAULT_KICK_TO_REASON",
            Command::DefaultKickToServer => "DEFAULT_KICK_TO_SERVER",
            Command::DefaultShoutPlayer => "DEFAULT_SHOUT_PLAYER",
            Command::DefaultShoutSpectator => "DEFAULT_SHOUT_SPECTATOR",
            Command::DisallowRenamePlayer => "DISALLOW_RENAME_PLAYER",
            Command::DisallowTeamChangePlayer => "DISALLOW_TEAM_CHANGE_PLAYER",
            Command::Dither => "DITHER",
            Command::DoublebindTime => "DOUBLEBIND_TIME",
            Command::DoublebindTimeOverride => "DOUBLEBIND_TIME_OVERRIDE",
            Command::EnableChat => "ENABLE_CHAT",
            Command::EnableFriends => "ENABLE_FRIENDS",
            Command::EnemyChatbotPenalty => "ENEMY_CHATBOT_PENALTY",
            Command::EnemyCurrenttimeInfluence => "ENEMY_CURRENTTIME_INFLUENCE",
            Command::EnemyDeadPenalty => "ENEMY_DEAD_PENALTY",
            Command::EnemySuicideTimeout => "ENEMY_SUICIDE_TIMEOUT",
            Command::EnemyTeammatePenalty => "ENEMY_TEAMMATE_PENALTY",
            Command::ExpectAckOnClientPlayback => "EXPECT_ACK_ON_CLIENT_PLAYBACK",
            Command::Explosion => "EXPLOSION",
            Command::ExplosionRadius => "EXPLOSION_RADIUS",
            Command::ExtraRoundTime => "EXTRA_ROUND_TIME",
            Command::FadeoutNameDelay => "FADEOUT_NAME_DELAY",
            Command::FailedAttempts => "FAILED_ATTEMPTS",
            Command::FastestLocx => "FASTEST_LOCX",
            Command::FastestLocy => "FASTEST_LOCY",
            Command::FastestSize => "FASTEST_SIZE",
            Command::FastForwardMaxstep => "FAST_FORWARD_MAXSTEP",
            Command::FastForwardMaxstepReal => "FAST_FORWARD_MAXSTEP_REAL",
            Command::FastForwardMaxstepRel => "FAST_FORWARD_MAXSTEP_REL",
            Command::FavNumPerTeamPlayer1 => "FAV_NUM_PER_TEAM_PLAYER_1",
            Command::FavNumPerTeamPlayer2 => "FAV_NUM_PER_TEAM_PLAYER_2",
            Command::FavNumPerTeamPlayer3 => "FAV_NUM_PER_TEAM_PLAYER_3",
            Command::FavNumPerTeamPlayer4 => "FAV_NUM_PER_TEAM_PLAYER_4",
            Command::FilterColorNames => "FILTER_COLOR_NAMES",
            Command::FilterColorServerNames => "FILTER_COLOR_SERVER_NAMES",
            Command::FilterColorStrings => "FILTER_COLOR_STRINGS",
            Command::FilterColorTeam => "FILTER_COLOR_TEAM",
            Command::FilterDarkColorNames => "FILTER_DARK_COLOR_NAMES",
            Command::FilterDarkColorServerNames => "FILTER_DARK_COLOR_SERVER_NAMES",
            Command::FilterDarkColorStrings => "FILTER_DARK_COLOR_STRINGS",
            Command::FilterDarkColorTeam => "FILTER_DARK_COLOR_TEAM",
            Command::FilterNameEnds => "FILTER_NAME_ENDS",
            Command::FilterNameMiddle => "FILTER_NAME_MIDDLE",
            Command::FinishType => "FINISH_TYPE",
            Command::FirstUse => "FIRST_USE",
            Command::FloorBlue => "FLOOR_BLUE",
            Command::FloorDetail => "FLOOR_DETAIL",
            Command::FloorGreen => "FLOOR_GREEN",
            Command::FloorMirror => "FLOOR_MIRROR",
            Command::FloorMirrorInt => "FLOOR_MIRROR_INT",
            Command::FloorRed => "FLOOR_RED",
            Command::FontSmallThresholdHeight => "FONT_SMALL_THRESHOLD_HEIGHT",
            Command::FontSmallThresholdWidth => "FONT_SMALL_THRESHOLD_WIDTH",
            Command::ForceTurtleMode => "FORCE_TURTLE_MODE",
            Command::FortressCollapseSpeed => "FORTRESS_COLLAPSE_SPEED",
            Command::FortressConquestDecayRate => "FORTRESS_CONQUEST_DECAY_RATE",
            Command::FortressConquestRate => "FORTRESS_CONQUEST_RATE",
            Command::FortressConquestTimeout => "FORTRESS_CONQUEST_TIMEOUT",
            Command::FortressDefendRate => "FORTRESS_DEFEND_RATE",
            Command::FortressHeldScore => "FORTRESS_HELD_SCORE",
            Command::Friend1 => "FRIEND_1",
            Command::Friend10 => "FRIEND_10",
            Command::Friend2 => "FRIEND_2",
            Command::Friend3 => "FRIEND_3",
            Command::Friend4 => "FRIEND_4",
            Command::Friend5 => "FRIEND_5",
            Command::Friend6 => "FRIEND_6",
            Command::Friend7 => "FRIEND_7",
            Command::Friend8 => "FRIEND_8",
            Command::Friend9 => "FRIEND_9",
            Command::Fullscreen => "FULLSCREEN",
            Command::FullscreenMessage => "FULLSCREEN_MESSAGE",
            Command::GameTimeout => "GAME_TIMEOUT",
            Command::GameType => "GAME_TYPE",
            Command::GlanceBackTooltip => "GLANCE_BACK_TOOLTIP",
            Command::GlanceLeftTooltip => "GLANCE_LEFT_TOOLTIP",
            Command::GlanceRightTooltip => "GLANCE_RIGHT_TOOLTIP",
            Command::GlobalId => "GLOBAL_ID",
            Command::GlExtensions => "GL_EXTENSIONS",
            Command::GlRenderer => "GL_RENDERER",
            Command::GlVendor => "GL_VENDOR",
            Command::GlVersion => "GL_VERSION",
            Command::GridSize => "GRID_SIZE",
            Command::GridSizeMoviepack => "GRID_SIZE_MOVIEPACK",
            Command::HashMethodBlacklist => "HASH_METHOD_BLACKLIST",
            Command::HelpIntroductoryBlurb => "HELP_INTRODUCTORY_BLURB",
            Command::HideIdentity1 => "HIDE_IDENTITY_1",
            Command::HideIdentity2 => "HIDE_IDENTITY_2",
            Command::HideIdentity3 => "HIDE_IDENTITY_3",
            Command::HideIdentity4 => "HIDE_IDENTITY_4",
            Command::HighRim => "HIGH_RIM",
            Command::HistorySizeChat => "HISTORY_SIZE_CHAT",
            Command::HistorySizeConsole => "HISTORY_SIZE_CONSOLE",
            Command::HudCacheThreshold => "HUD_CACHE_THRESHOLD",
            Command::HudMaxWidth => "HUD_MAX_WIDTH",
            Command::IdleKickTime => "IDLE_KICK_TIME",
            Command::IdleRemoveTime => "IDLE_REMOVE_TIME",
            Command::Include => "INCLUDE",
            Command::InfinityPlane => "INFINITY_PLANE",
            Command::IngameMenuTooltip => "INGAME_MENU_TOOLTIP",
            Command::InstantChatString1_1 => "INSTANT_CHAT_STRING_1_1",
            Command::InstantChatString1_10 => "INSTANT_CHAT_STRING_1_10",
            Command::InstantChatString1_11 => "INSTANT_CHAT_STRING_1_11",
            Command::InstantChatString1_12 => "INSTANT_CHAT_STRING_1_12",
            Command::InstantChatString1_13 => "INSTANT_CHAT_STRING_1_13",
            Command::InstantChatString1_14 => "INSTANT_CHAT_STRING_1_14",
            Command::InstantChatString1_15 => "INSTANT_CHAT_STRING_1_15",
            Command::InstantChatString1_16 => "INSTANT_CHAT_STRING_1_16",
            Command::InstantChatString1_17 => "INSTANT_CHAT_STRING_1_17",
            Command::InstantChatString1_18 => "INSTANT_CHAT_STRING_1_18",
            Command::InstantChatString1_19 => "INSTANT_CHAT_STRING_1_19",
            Command::InstantChatString1_2 => "INSTANT_CHAT_STRING_1_2",
            Command::InstantChatString1_20 => "INSTANT_CHAT_STRING_1_20",
            Command::InstantChatString1_21 => "INSTANT_CHAT_STRING_1_21",
            Command::InstantChatString1_22 => "INSTANT_CHAT_STRING_1_22",
            Command::InstantChatString1_23 => "INSTANT_CHAT_STRING_1_23",
            Command::InstantChatString1_24 => "INSTANT_CHAT_STRING_1_24",
            Command::InstantChatString1_25 => "INSTANT_CHAT_STRING_1_25",
            Command::InstantChatString1_3 => "INSTANT_CHAT_STRING_1_3",
            Command::InstantChatString1_4 => "INSTANT_CHAT_STRING_1_4",
            Command::InstantChatString1_5 => "INSTANT_CHAT_STRING_1_5",
            Command::InstantChatString1_6 => "INSTANT_CHAT_STRING_1_6",
            Command::InstantChatString1_7 => "INSTANT_CHAT_STRING_1_7",
            Command::InstantChatString1_8 => "INSTANT_CHAT_STRING_1_8",
            Command::InstantChatString1_9 => "INSTANT_CHAT_STRING_1_9",
            Command::InstantChatString2_1 => "INSTANT_CHAT_STRING_2_1",
            Command::InstantChatString2_10 => "INSTANT_CHAT_STRING_2_10",
            Command::InstantChatString2_11 => "INSTANT_CHAT_STRING_2_11",
            Command::InstantChatString2_12 => "INSTANT_CHAT_STRING_2_12",
            Command::InstantChatString2_13 => "INSTANT_CHAT_STRING_2_13",
            Command::InstantChatString2_14 => "INSTANT_CHAT_STRING_2_14",
            Command::InstantChatString2_15 => "INSTANT_CHAT_STRING_2_15",
            Command::InstantChatString2_16 => "INSTANT_CHAT_STRING_2_16",
            Command::InstantChatString2_17 => "INSTANT_CHAT_STRING_2_17",
            Command::InstantChatString2_18 => "INSTANT_CHAT_STRING_2_18",
            Command::InstantChatString2_19 => "INSTANT_CHAT_STRING_2_19",
            Command::InstantChatString2_2 => "INSTANT_CHAT_STRING_2_2",
            Command::InstantChatString2_20 => "INSTANT_CHAT_STRING_2_20",
            Command::InstantChatString2_21 => "INSTANT_CHAT_STRING_2_21",
            Command::InstantChatString2_22 => "INSTANT_CHAT_STRING_2_22",
            Command::InstantChatString2_23 => "INSTANT_CHAT_STRING_2_23",
            Command::InstantChatString2_24 => "INSTANT_CHAT_STRING_2_24",
            Command::InstantChatString2_25 => "INSTANT_CHAT_STRING_2_25",
            Command::InstantChatString2_3 => "INSTANT_CHAT_STRING_2_3",
            Command::InstantChatString2_4 => "INSTANT_CHAT_STRING_2_4",
            Command::InstantChatString2_5 => "INSTANT_CHAT_STRING_2_5",
            Command::InstantChatString2_6 => "INSTANT_CHAT_STRING_2_6",
            Command::InstantChatString2_7 => "INSTANT_CHAT_STRING_2_7",
            Command::InstantChatString2_8 => "INSTANT_CHAT_STRING_2_8",
            Command::InstantChatString2_9 => "INSTANT_CHAT_STRING_2_9",
            Command::InstantChatString3_1 => "INSTANT_CHAT_STRING_3_1",
            Command::InstantChatString3_10 => "INSTANT_CHAT_STRING_3_10",
            Command::InstantChatString3_11 => "INSTANT_CHAT_STRING_3_11",
            Command::InstantChatString3_12 => "INSTANT_CHAT_STRING_3_12",
            Command::InstantChatString3_13 => "INSTANT_CHAT_STRING_3_13",
            Command::InstantChatString3_14 => "INSTANT_CHAT_STRING_3_14",
            Command::InstantChatString3_15 => "INSTANT_CHAT_STRING_3_15",
            Command::InstantChatString3_16 => "INSTANT_CHAT_STRING_3_16",
            Command::InstantChatString3_17 => "INSTANT_CHAT_STRING_3_17",
            Command::InstantChatString3_18 => "INSTANT_CHAT_STRING_3_18",
            Command::InstantChatString3_19 => "INSTANT_CHAT_STRING_3_19",
            Command::InstantChatString3_2 => "INSTANT_CHAT_STRING_3_2",
            Command::InstantChatString3_20 => "INSTANT_CHAT_STRING_3_20",
            Command::InstantChatString3_21 => "INSTANT_CHAT_STRING_3_21",
            Command::InstantChatString3_22 => "INSTANT_CHAT_STRING_3_22",
            Command::InstantChatString3_23 => "INSTANT_CHAT_STRING_3_23",
            Command::InstantChatString3_24 => "INSTANT_CHAT_STRING_3_24",
            Command::InstantChatString3_25 => "INSTANT_CHAT_STRING_3_25",
            Command::InstantChatString3_3 => "INSTANT_CHAT_STRING_3_3",
            Command::InstantChatString3_4 => "INSTANT_CHAT_STRING_3_4",
            Command::InstantChatString3_5 => "INSTANT_CHAT_STRING_3_5",
            Command::InstantChatString3_6 => "INSTANT_CHAT_STRING_3_6",
            Command::InstantChatString3_7 => "INSTANT_CHAT_STRING_3_7",
            Command::InstantChatString3_8 => "INSTANT_CHAT_STRING_3_8",
            Command::InstantChatString3_9 => "INSTANT_CHAT_STRING_3_9",
            Command::InstantChatString4_1 => "INSTANT_CHAT_STRING_4_1",
            Command::InstantChatString4_10 => "INSTANT_CHAT_STRING_4_10",
            Command::InstantChatString4_11 => "INSTANT_CHAT_STRING_4_11",
            Command::InstantChatString4_12 => "INSTANT_CHAT_STRING_4_12",
            Command::InstantChatString4_13 => "INSTANT_CHAT_STRING_4_13",
            Command::InstantChatString4_14 => "INSTANT_CHAT_STRING_4_14",
            Command::InstantChatString4_15 => "INSTANT_CHAT_STRING_4_15",
            Command::InstantChatString4_16 => "INSTANT_CHAT_STRING_4_16",
            Command::InstantChatString4_17 => "INSTANT_CHAT_STRING_4_17",
            Command::InstantChatString4_18 => "INSTANT_CHAT_STRING_4_18",
            Command::InstantChatString4_19 => "INSTANT_CHAT_STRING_4_19",
            Command::InstantChatString4_2 => "INSTANT_CHAT_STRING_4_2",
            Command::InstantChatString4_20 => "INSTANT_CHAT_STRING_4_20",
            Command::InstantChatString4_21 => "INSTANT_CHAT_STRING_4_21",
            Command::InstantChatString4_22 => "INSTANT_CHAT_STRING_4_22",
            Command::InstantChatString4_23 => "INSTANT_CHAT_STRING_4_23",
            Command::InstantChatString4_24 => "INSTANT_CHAT_STRING_4_24",
            Command::InstantChatString4_25 => "INSTANT_CHAT_STRING_4_25",
            Command::InstantChatString4_3 => "INSTANT_CHAT_STRING_4_3",
            Command::InstantChatString4_4 => "INSTANT_CHAT_STRING_4_4",
            Command::InstantChatString4_5 => "INSTANT_CHAT_STRING_4_5",
            Command::InstantChatString4_6 => "INSTANT_CHAT_STRING_4_6",
            Command::InstantChatString4_7 => "INSTANT_CHAT_STRING_4_7",
            Command::InstantChatString4_8 => "INSTANT_CHAT_STRING_4_8",
            Command::InstantChatString4_9 => "INSTANT_CHAT_STRING_4_9",
            Command::KeepPlayerSlot => "KEEP_PLAYER_SLOT",
            Command::KeepWindowActive => "KEEP_WINDOW_ACTIVE",
            Command::Keyboard => "KEYBOARD",
            Command::Kick => "KICK",
            Command::KickTo => "KICK_TO",
            Command::Kill => "KILL",
            Command::LadderlogDecorateTimestamp => "LADDERLOG_DECORATE_TIMESTAMP",
            Command::LadderlogGameTimeInterval => "LADDERLOG_GAME_TIME_INTERVAL",
            Command::LadderlogWriteAll => "LADDERLOG_WRITE_ALL",
            Command::LadderlogWriteAuthorityBlurb => "LADDERLOG_WRITE_AUTHORITY_BLURB",
            Command::LadderlogWriteBasezoneConquered => "LADDERLOG_WRITE_BASEZONE_CONQUERED",
            Command::LadderlogWriteBasezoneConquerer => "LADDERLOG_WRITE_BASEZONE_CONQUERER",
            Command::LadderlogWriteChat => "LADDERLOG_WRITE_CHAT",
            Command::LadderlogWriteDeathFrag => "LADDERLOG_WRITE_DEATH_FRAG",
            Command::LadderlogWriteDeathSuicide => "LADDERLOG_WRITE_DEATH_SUICIDE",
            Command::LadderlogWriteDeathTeamkill => "LADDERLOG_WRITE_DEATH_TEAMKILL",
            Command::LadderlogWriteEncoding => "LADDERLOG_WRITE_ENCODING",
            Command::LadderlogWriteGameEnd => "LADDERLOG_WRITE_GAME_END",
            Command::LadderlogWriteGameTime => "LADDERLOG_WRITE_GAME_TIME",
            Command::LadderlogWriteMatchWinner => "LADDERLOG_WRITE_MATCH_WINNER",
            Command::LadderlogWriteNewMatch => "LADDERLOG_WRITE_NEW_MATCH",
            Command::LadderlogWriteNewRound => "LADDERLOG_WRITE_NEW_ROUND",
            Command::LadderlogWriteNumHumans => "LADDERLOG_WRITE_NUM_HUMANS",
            Command::LadderlogWriteOnlinePlayer => "LADDERLOG_WRITE_ONLINE_PLAYER",
            Command::LadderlogWritePlayerEntered => "LADDERLOG_WRITE_PLAYER_ENTERED",
            Command::LadderlogWritePlayerLeft => "LADDERLOG_WRITE_PLAYER_LEFT",
            Command::LadderlogWritePlayerRenamed => "LADDERLOG_WRITE_PLAYER_RENAMED",
            Command::LadderlogWritePositions => "LADDERLOG_WRITE_POSITIONS",
            Command::LadderlogWriteRoundScore => "LADDERLOG_WRITE_ROUND_SCORE",
            Command::LadderlogWriteRoundScoreTeam => "LADDERLOG_WRITE_ROUND_SCORE_TEAM",
            Command::LadderlogWriteRoundWinner => "LADDERLOG_WRITE_ROUND_WINNER",
            Command::LadderlogWriteSacrifice => "LADDERLOG_WRITE_SACRIFICE",
            Command::LadderlogWriteTeamCreated => "LADDERLOG_WRITE_TEAM_CREATED",
            Command::LadderlogWriteTeamDestroyed => "LADDERLOG_WRITE_TEAM_DESTROYED",
            Command::LadderlogWriteTeamPlayerAdded => "LADDERLOG_WRITE_TEAM_PLAYER_ADDED",
            Command::LadderlogWriteTeamPlayerRemoved => "LADDERLOG_WRITE_TEAM_PLAYER_REMOVED",
            Command::LadderlogWriteTeamRenamed => "LADDERLOG_WRITE_TEAM_RENAMED",
            Command::LadderlogWriteWaitForExternalScript => {
                "LADDERLOG_WRITE_WAIT_FOR_EXTERNAL_SCRIPT"
            }
            Command::LadderGainExtra => "LADDER_GAIN_EXTRA",
            Command::LadderLoseMinOnLoad => "LADDER_LOSE_MIN_ON_LOAD",
            Command::LadderLosePercentOnLoad => "LADDER_LOSE_PERCENT_ON_LOAD",
            Command::LadderMinBet => "LADDER_MIN_BET",
            Command::LadderPercentBet => "LADDER_PERCENT_BET",
            Command::LadderTax => "LADDER_TAX",
            Command::LagCredit => "LAG_CREDIT",
            Command::LagCreditSingle => "LAG_CREDIT_SINGLE",
            Command::LagCreditTime => "LAG_CREDIT_TIME",
            Command::LagCreditVariance => "LAG_CREDIT_VARIANCE",
            Command::LagFastTime => "LAG_FAST_TIME",
            Command::LagFastWeight => "LAG_FAST_WEIGHT",
            Command::LagFrequencyThreshold => "LAG_FREQUENCY_THRESHOLD",
            Command::LagMaxSpeedupTimer => "LAG_MAX_SPEEDUP_TIMER",
            Command::LagOffsetClient => "LAG_OFFSET_CLIENT",
            Command::LagOffsetLegacy => "LAG_OFFSET_LEGACY",
            Command::LagOffsetServer => "LAG_OFFSET_SERVER",
            Command::LagOMeter => "LAG_O_METER",
            Command::LagOMeterBlend => "LAG_O_METER_BLEND",
            Command::LagOMeterScale => "LAG_O_METER_SCALE",
            Command::LagOMeterThreshold => "LAG_O_METER_THRESHOLD",
            Command::LagOMeterUseOld => "LAG_O_METER_USE_OLD",
            Command::LagSlowTime => "LAG_SLOW_TIME",
            Command::LagSlowWeight => "LAG_SLOW_WEIGHT",
            Command::LagSweetSpot => "LAG_SWEET_SPOT",
            Command::LagThreshold => "LAG_THRESHOLD",
            Command::LanguageFirst => "LANGUAGE_FIRST",
            Command::LanguageSecond => "LANGUAGE_SECOND",
            Command::LastChatBreakTime => "LAST_CHAT_BREAK_TIME",
            Command::LastCheckErrors => "LAST_CHECK_ERRORS",
            Command::LastColordepth => "LAST_COLORDEPTH",
            Command::LastFullscreen => "LAST_FULLSCREEN",
            Command::LastZdepth => "LAST_ZDEPTH",
            Command::LegacyLogNames => "LEGACY_LOG_NAMES",
            Command::LimitRounds => "LIMIT_ROUNDS",
            Command::LimitScore => "LIMIT_SCORE",
            Command::LimitTime => "LIMIT_TIME",
            Command::LocalTeam => "LOCAL_TEAM",
            Command::LocalUser => "LOCAL_USER",
            Command::LowerSky => "LOWER_SKY",
            Command::MapFile => "MAP_FILE",
            Command::MapFileOverride => "MAP_FILE_OVERRIDE",
            Command::MapUri => "MAP_URI",
            Command::MaxClients => "MAX_CLIENTS",
            Command::MaxClientsSameIpHard => "MAX_CLIENTS_SAME_IP_HARD",
            Command::MaxClientsSameIpSoft => "MAX_CLIENTS_SAME_IP_SOFT",
            Command::MaxInRate => "MAX_IN_RATE",
            Command::MaxOutRate => "MAX_OUT_RATE",
            Command::MaxPlayersSameIp => "MAX_PLAYERS_SAME_IP",
            Command::MaxProtocolVersion => "MAX_PROTOCOL_VERSION",
            Command::MaxVotes => "MAX_VOTES",
            Command::MaxVotesPerVoter => "MAX_VOTES_PER_VOTER",
            Command::Md5Prefix => "MD5_PREFIX",
            Command::Md5Suffix => "MD5_SUFFIX",
            Command::MessageOfDay => "MESSAGE_OF_DAY",
            Command::MessageOfDayTimeout => "MESSAGE_OF_DAY_TIMEOUT",
            Command::MinPlayers => "MIN_PLAYERS",
            Command::MinPlayTimeOnline => "MIN_PLAY_TIME_ONLINE",
            Command::MinPlayTimeTeam => "MIN_PLAY_TIME_TEAM",
            Command::MinPlayTimeTotal => "MIN_PLAY_TIME_TOTAL",
            Command::MinProtocolVersion => "MIN_PROTOCOL_VERSION",
            Command::MinVoters => "MIN_VOTERS",
            Command::MouseGrab => "MOUSE_GRAB",
            Command::MoveTo => "MOVE_TO",
            Command::Moviepack => "MOVIEPACK",
            Command::MoviepackFloorBlue => "MOVIEPACK_FLOOR_BLUE",
            Command::MoviepackFloorGreen => "MOVIEPACK_FLOOR_GREEN",
            Command::MoviepackFloorRed => "MOVIEPACK_FLOOR_RED",
            Command::MoviepackRimWallStretchX => "MOVIEPACK_RIM_WALL_STRETCH_X",
            Command::MoviepackRimWallStretchY => "MOVIEPACK_RIM_WALL_STRETCH_Y",
            Command::MoviepackWallStretch => "MOVIEPACK_WALL_STRETCH",
            Command::NameTeamAfterPlayer1 => "NAME_TEAM_AFTER_PLAYER_1",
            Command::NameTeamAfterPlayer2 => "NAME_TEAM_AFTER_PLAYER_2",
            Command::NameTeamAfterPlayer3 => "NAME_TEAM_AFTER_PLAYER_3",
            Command::NameTeamAfterPlayer4 => "NAME_TEAM_AFTER_PLAYER_4",
            Command::NetworkAutobanFactor => "NETWORK_AUTOBAN_FACTOR",
            Command::NetworkAutobanMaxKph => "NETWORK_AUTOBAN_MAX_KPH",
            Command::NetworkAutobanOffset => "NETWORK_AUTOBAN_OFFSET",
            Command::NetworkMinBan => "NETWORK_MIN_BAN",
            Command::NetworkSpectatorTime => "NETWORK_SPECTATOR_TIME",
            Command::NewFeatureDelay => "NEW_FEATURE_DELAY",
            Command::NewTeamAllowed => "NEW_TEAM_ALLOWED",
            Command::NumAis => "NUM_AIS",
            Command::Password => "PASSWORD",
            Command::PasswordStorage => "PASSWORD_STORAGE",
            Command::PingCharity => "PING_CHARITY",
            Command::PingCharityMax => "PING_CHARITY_MAX",
            Command::PingCharityMin => "PING_CHARITY_MIN",
            Command::PingCharityServer => "PING_CHARITY_SERVER",
            Command::PingFloodGlobal => "PING_FLOOD_GLOBAL",
            Command::PingFloodTime10 => "PING_FLOOD_TIME_10",
            Command::PingFloodTime100 => "PING_FLOOD_TIME_100",
            Command::PingFloodTime20 => "PING_FLOOD_TIME_20",
            Command::PingFloodTime50 => "PING_FLOOD_TIME_50",
            Command::PingLocx => "PING_LOCX",
            Command::PingLocy => "PING_LOCY",
            Command::PingSize => "PING_SIZE",
            Command::Players => "PLAYERS",
            Command::Player1 => "PLAYER_1",
            Command::Player2 => "PLAYER_2",
            Command::Player3 => "PLAYER_3",
            Command::Player4 => "PLAYER_4",
            Command::PlayerChatWaitFraction => "PLAYER_CHAT_WAIT_FRACTION",
            Command::PlayerChatWaitMax => "PLAYER_CHAT_WAIT_MAX",
            Command::PlayerChatWaitSingle => "PLAYER_CHAT_WAIT_SINGLE",
            Command::PlayerChatWaitTeamleader => "PLAYER_CHAT_WAIT_TEAMLEADER",
            Command::PlayerListHiddenPlayerPrefix => "PLAYER_LIST_HIDDEN_PLAYER_PREFIX",
            Command::PlayerMessage => "PLAYER_MESSAGE",
            Command::PlayerRandomColor => "PLAYER_RANDOM_COLOR",
            Command::PlayTimeOnline => "PLAY_TIME_ONLINE",
            Command::PlayTimeTeam => "PLAY_TIME_TEAM",
            Command::PlayTimeTotal => "PLAY_TIME_TOTAL",
            Command::PngScreenshot => "PNG_SCREENSHOT",
            Command::PredictObjects => "PREDICT_OBJECTS",
            Command::PrefixSpamEnable => "PREFIX_SPAM_ENABLE",
            Command::PrefixSpamLengthMultiplier => "PREFIX_SPAM_LENGTH_MULTIPLIER",
            Command::PrefixSpamNumberColorCodesMultiplier => {
                "PREFIX_SPAM_NUMBER_COLOR_CODES_MULTIPLIER"
            }
            Command::PrefixSpamNumberKnownPrefixesMultiplier => {
                "PREFIX_SPAM_NUMBER_KNOWN_PREFIXES_MULTIPLIER"
            }
            Command::PrefixSpamRequiredScore => "PREFIX_SPAM_REQUIRED_SCORE",
            Command::PrefixSpamStartColorMultiplier => "PREFIX_SPAM_START_COLOR_MULTIPLIER",
            Command::PrefixSpamTimeoutMultiplier => "PREFIX_SPAM_TIMEOUT_MULTIPLIER",
            Command::ProtectSensitiveFiles => "PROTECT_SENSITIVE_FILES",
            Command::RealArenaSizeFactor => "REAL_ARENA_SIZE_FACTOR",
            Command::RealCycleSpeedFactor => "REAL_CYCLE_SPEED_FACTOR",
            Command::RecordingDebuglevel => "RECORDING_DEBUGLEVEL",
            Command::RecordTurtleMode => "RECORD_TURTLE_MODE",
            Command::RemoveHelpTopic => "REMOVE_HELP_TOPIC",
            Command::Rename => "RENAME",
            Command::ReserveScreenName => "RESERVE_SCREEN_NAME",
            Command::ResourceRepositoryClient => "RESOURCE_REPOSITORY_CLIENT",
            Command::ResourceRepositoryServer => "RESOURCE_REPOSITORY_SERVER",
            Command::RimWallStretchX => "RIM_WALL_STRETCH_X",
            Command::RimWallStretchY => "RIM_WALL_STRETCH_Y",
            Command::RimWallWrapY => "RIM_WALL_WRAP_Y",
            Command::Rinclude => "RINCLUDE",
            Command::RoundCenterMessage => "ROUND_CENTER_MESSAGE",
            Command::RoundConsoleMessage => "ROUND_CONSOLE_MESSAGE",
            Command::RubberGaugeLocx => "RUBBER_GAUGE_LOCX",
            Command::RubberGaugeLocy => "RUBBER_GAUGE_LOCY",
            Command::RubberGaugeSize => "RUBBER_GAUGE_SIZE",
            Command::SavedInVersion => "SAVED_IN_VERSION",
            Command::Say => "SAY",
            Command::ScoreDeathzone => "SCORE_DEATHZONE",
            Command::ScoreDie => "SCORE_DIE",
            Command::ScoreHole => "SCORE_HOLE",
            Command::ScoreKill => "SCORE_KILL",
            Command::ScoreLocx => "SCORE_LOCX",
            Command::ScoreLocy => "SCORE_LOCY",
            Command::ScoreSize => "SCORE_SIZE",
            Command::ScoreSuicide => "SCORE_SUICIDE",
            Command::ScoreSurvive => "SCORE_SURVIVE",
            Command::ScoreWin => "SCORE_WIN",
            Command::ServerDns => "SERVER_DNS",
            Command::ServerIp => "SERVER_IP",
            Command::ServerName => "SERVER_NAME",
            Command::ServerPort => "SERVER_PORT",
            Command::SettingLegacyBehaviorAnnoying => "SETTING_LEGACY_BEHAVIOR_ANNOYING",
            Command::SettingLegacyBehaviorBreaking => "SETTING_LEGACY_BEHAVIOR_BREAKING",
            Command::SettingLegacyBehaviorBumpy => "SETTING_LEGACY_BEHAVIOR_BUMPY",
            Command::SettingLegacyBehaviorCheating => "SETTING_LEGACY_BEHAVIOR_CHEATING",
            Command::SettingLegacyBehaviorVisual => "SETTING_LEGACY_BEHAVIOR_VISUAL",
            Command::ShowAlive => "SHOW_ALIVE",
            Command::ShowBrake => "SHOW_BRAKE",
            Command::ShowFastest => "SHOW_FASTEST",
            Command::ShowFps => "SHOW_FPS",
            Command::ShowHud => "SHOW_HUD",
            Command::ShowOwnIp => "SHOW_OWN_IP",
            Command::ShowOwnName => "SHOW_OWN_NAME",
            Command::ShowPing => "SHOW_PING",
            Command::ShowRecordingTime => "SHOW_RECORDING_TIME",
            Command::ShowRubber => "SHOW_RUBBER",
            Command::ShowScore => "SHOW_SCORE",
            Command::ShowSpeed => "SHOW_SPEED",
            Command::ShowTime => "SHOW_TIME",
            Command::ShowTime24 => "SHOW_TIME_24",
            Command::ShuffleSpamMessagesPerRound => "SHUFFLE_SPAM_MESSAGES_PER_ROUND",
            Command::Silence => "SILENCE",
            Command::SilenceDefault => "SILENCE_DEFAULT",
            Command::SimpleTrail => "SIMPLE_TRAIL",
            Command::Sinclude => "SINCLUDE",
            Command::SizeFactor => "SIZE_FACTOR",
            Command::SkyWobble => "SKY_WOBBLE",
            Command::Slap => "SLAP",
            Command::SmartGlanceCustom1 => "SMART_GLANCE_CUSTOM_1",
            Command::SmartGlanceCustom2 => "SMART_GLANCE_CUSTOM_2",
            Command::SmartGlanceCustom3 => "SMART_GLANCE_CUSTOM_3",
            Command::SmartGlanceCustom4 => "SMART_GLANCE_CUSTOM_4",
            Command::SmoothShading => "SMOOTH_SHADING",
            Command::SoftwareRenderer => "SOFTWARE_RENDERER",
            Command::SoundBufferShift => "SOUND_BUFFER_SHIFT",
            Command::SoundQuality => "SOUND_QUALITY",
            Command::SoundSources => "SOUND_SOURCES",
            Command::SpamAutokick => "SPAM_AUTOKICK",
            Command::SpamAutokickCount => "SPAM_AUTOKICK_COUNT",
            Command::SpamMaxlen => "SPAM_MAXLEN",
            Command::SpamMaxlenOverride => "SPAM_MAXLEN_OVERRIDE",
            Command::SpamPenalty => "SPAM_PENALTY",
            Command::SpamProtection => "SPAM_PROTECTION",
            Command::SpamProtectionChat => "SPAM_PROTECTION_CHAT",
            Command::SpamProtectionRepeat => "SPAM_PROTECTION_REPEAT",
            Command::SpamProtectionVote => "SPAM_PROTECTION_VOTE",
            Command::Sparks => "SPARKS",
            Command::SpawnWingmenBack => "SPAWN_WINGMEN_BACK",
            Command::SpawnWingmenSide => "SPAWN_WINGMEN_SIDE",
            Command::SpectatorMode1 => "SPECTATOR_MODE_1",
            Command::SpectatorMode2 => "SPECTATOR_MODE_2",
            Command::SpectatorMode3 => "SPECTATOR_MODE_3",
            Command::SpectatorMode4 => "SPECTATOR_MODE_4",
            Command::SpeedFactor => "SPEED_FACTOR",
            Command::SpeedGaugeLocx => "SPEED_GAUGE_LOCX",
            Command::SpeedGaugeLocy => "SPEED_GAUGE_LOCY",
            Command::SpeedGaugeSize => "SPEED_GAUGE_SIZE",
            Command::SpAiIq => "SP_AI_IQ",
            Command::SpAutoAis => "SP_AUTO_AIS",
            Command::SpAutoIq => "SP_AUTO_IQ",
            Command::SpExplosionRadius => "SP_EXPLOSION_RADIUS",
            Command::SpFinishType => "SP_FINISH_TYPE",
            Command::SpGameType => "SP_GAME_TYPE",
            Command::SpLimitRounds => "SP_LIMIT_ROUNDS",
            Command::SpLimitScore => "SP_LIMIT_SCORE",
            Command::SpLimitTime => "SP_LIMIT_TIME",
            Command::SpMinPlayers => "SP_MIN_PLAYERS",
            Command::SpNumAis => "SP_NUM_AIS",
            Command::SpScoreWin => "SP_SCORE_WIN",
            Command::SpSizeFactor => "SP_SIZE_FACTOR",
            Command::SpSpeedFactor => "SP_SPEED_FACTOR",
            Command::SpTeamsMax => "SP_TEAMS_MAX",
            Command::SpTeamsMin => "SP_TEAMS_MIN",
            Command::SpTeamBalanceOnQuit => "SP_TEAM_BALANCE_ON_QUIT",
            Command::SpTeamBalanceWithAis => "SP_TEAM_BALANCE_WITH_AIS",
            Command::SpTeamMaxImbalance => "SP_TEAM_MAX_IMBALANCE",
            Command::SpTeamMaxPlayers => "SP_TEAM_MAX_PLAYERS",
            Command::SpTeamMinPlayers => "SP_TEAM_MIN_PLAYERS",
            Command::SpWallsLength => "SP_WALLS_LENGTH",
            Command::SpWallsStayUpDelay => "SP_WALLS_STAY_UP_DELAY",
            Command::SpWinZoneMinLastDeath => "SP_WIN_ZONE_MIN_LAST_DEATH",
            Command::SpWinZoneMinRoundTime => "SP_WIN_ZONE_MIN_ROUND_TIME",
            Command::StartCam1 => "START_CAM_1",
            Command::StartCam2 => "START_CAM_2",
            Command::StartCam3 => "START_CAM_3",
            Command::StartCam4 => "START_CAM_4",
            Command::StartFov1 => "START_FOV_1",
            Command::StartFov2 => "START_FOV_2",
            Command::StartFov3 => "START_FOV_3",
            Command::StartFov4 => "START_FOV_4",
            Command::StartNewMatch => "START_NEW_MATCH",
            Command::StopRecording => "STOP_RECORDING",
            Command::Suspend => "SUSPEND",
            Command::SuspendDefaultRounds => "SUSPEND_DEFAULT_ROUNDS",
            Command::SwapMode => "SWAP_MODE",
            Command::SwitchViewTooltip => "SWITCH_VIEW_TOOLTIP",
            Command::TalkToMaster => "TALK_TO_MASTER",
            Command::Teams => "TEAMS",
            Command::TeamsMax => "TEAMS_MAX",
            Command::TeamsMin => "TEAMS_MIN",
            Command::TeamBalanceOnQuit => "TEAM_BALANCE_ON_QUIT",
            Command::TeamBalanceWithAis => "TEAM_BALANCE_WITH_AIS",
            Command::TeamBlue1 => "TEAM_BLUE_1",
            Command::TeamBlue2 => "TEAM_BLUE_2",
            Command::TeamBlue3 => "TEAM_BLUE_3",
            Command::TeamBlue4 => "TEAM_BLUE_4",
            Command::TeamBlue5 => "TEAM_BLUE_5",
            Command::TeamBlue6 => "TEAM_BLUE_6",
            Command::TeamBlue7 => "TEAM_BLUE_7",
            Command::TeamBlue8 => "TEAM_BLUE_8",
            Command::TeamCenterIsBoss => "TEAM_CENTER_IS_BOSS",
            Command::TeamEliminationMode => "TEAM_ELIMINATION_MODE",
            Command::TeamGreen1 => "TEAM_GREEN_1",
            Command::TeamGreen2 => "TEAM_GREEN_2",
            Command::TeamGreen3 => "TEAM_GREEN_3",
            Command::TeamGreen4 => "TEAM_GREEN_4",
            Command::TeamGreen5 => "TEAM_GREEN_5",
            Command::TeamGreen6 => "TEAM_GREEN_6",
            Command::TeamGreen7 => "TEAM_GREEN_7",
            Command::TeamGreen8 => "TEAM_GREEN_8",
            Command::TeamMaxImbalance => "TEAM_MAX_IMBALANCE",
            Command::TeamMaxPlayers => "TEAM_MAX_PLAYERS",
            Command::TeamMinPlayers => "TEAM_MIN_PLAYERS",
            Command::TeamName1 => "TEAM_NAME_1",
            Command::TeamName2 => "TEAM_NAME_2",
            Command::TeamName3 => "TEAM_NAME_3",
            Command::TeamName4 => "TEAM_NAME_4",
            Command::TeamName5 => "TEAM_NAME_5",
            Command::TeamName6 => "TEAM_NAME_6",
            Command::TeamName7 => "TEAM_NAME_7",
            Command::TeamName8 => "TEAM_NAME_8",
            Command::TeamRed1 => "TEAM_RED_1",
            Command::TeamRed2 => "TEAM_RED_2",
            Command::TeamRed3 => "TEAM_RED_3",
            Command::TeamRed4 => "TEAM_RED_4",
            Command::TeamRed5 => "TEAM_RED_5",
            Command::TeamRed6 => "TEAM_RED_6",
            Command::TeamRed7 => "TEAM_RED_7",
            Command::TeamRed8 => "TEAM_RED_8",
            Command::TexturesHi => "TEXTURES_HI",
            Command::TextureMode0 => "TEXTURE_MODE_0",
            Command::TextureMode1 => "TEXTURE_MODE_1",
            Command::TextureMode2 => "TEXTURE_MODE_2",
            Command::TextureMode3 => "TEXTURE_MODE_3",
            Command::TextOut => "TEXT_OUT",
            Command::TimebotActionHigh => "TIMEBOT_ACTION_HIGH",
            Command::TimebotActionMax => "TIMEBOT_ACTION_MAX",
            Command::TimebotActionMedium => "TIMEBOT_ACTION_MEDIUM",
            Command::TimebotKickSeverity => "TIMEBOT_KICK_SEVERITY",
            Command::TimebotSensitivity => "TIMEBOT_SENSITIVITY",
            Command::TitleOfDay => "TITLE_OF_DAY",
            Command::ToggleSpectatorTooltip => "TOGGLE_SPECTATOR_TOOLTIP",
            Command::TopologyPolice => "TOPOLOGY_POLICE",
            Command::TopologyPoliceParallel => "TOPOLOGY_POLICE_PARALLEL",
            Command::TrustLan => "TRUST_LAN",
            Command::UnbanIp => "UNBAN_IP",
            Command::UnbanUser => "UNBAN_USER",
            Command::UnlockAllTeams => "UNLOCK_ALL_TEAMS",
            Command::Unsilence => "UNSILENCE",
            Command::Unsuspend => "UNSUSPEND",
            Command::UpperSky => "UPPER_SKY",
            Command::Url => "URL",
            Command::User1 => "USER_1",
            Command::User2 => "USER_2",
            Command::User3 => "USER_3",
            Command::User4 => "USER_4",
            Command::UserAlias => "USER_ALIAS",
            Command::UserLevel => "USER_LEVEL",
            Command::UserRemove => "USER_REMOVE",
            Command::UseDisplaylists => "USE_DISPLAYLISTS",
            Command::ViewportConf => "VIEWPORT_CONF",
            Command::ViewportToPlayer1 => "VIEWPORT_TO_PLAYER_1",
            Command::ViewportToPlayer2 => "VIEWPORT_TO_PLAYER_2",
            Command::ViewportToPlayer3 => "VIEWPORT_TO_PLAYER_3",
            Command::ViewportToPlayer4 => "VIEWPORT_TO_PLAYER_4",
            Command::Voice => "VOICE",
            Command::VotesCancel => "VOTES_CANCEL",
            Command::VotesSuspend => "VOTES_SUSPEND",
            Command::VotesSuspendDefault => "VOTES_SUSPEND_DEFAULT",
            Command::VotesUnsuspend => "VOTES_UNSUSPEND",
            Command::VoteKickReason => "VOTE_KICK_REASON",
            Command::VoteKickToPort => "VOTE_KICK_TO_PORT",
            Command::VoteKickToServer => "VOTE_KICK_TO_SERVER",
            Command::VoteUseServerControlledKick => "VOTE_USE_SERVER_CONTROLLED_KICK",
            Command::VotingBias => "VOTING_BIAS",
            Command::VotingBiasCommand => "VOTING_BIAS_COMMAND",
            Command::VotingBiasInclude => "VOTING_BIAS_INCLUDE",
            Command::VotingBiasKick => "VOTING_BIAS_KICK",
            Command::VotingBiasSilence => "VOTING_BIAS_SILENCE",
            Command::VotingBiasSuspend => "VOTING_BIAS_SUSPEND",
            Command::VotingBiasVoice => "VOTING_BIAS_VOICE",
            Command::VotingDecay => "VOTING_DECAY",
            Command::VotingHarmTime => "VOTING_HARM_TIME",
            Command::VotingKickMinharm => "VOTING_KICK_MINHARM",
            Command::VotingKickTime => "VOTING_KICK_TIME",
            Command::VotingMaturity => "VOTING_MATURITY",
            Command::VotingPrivacy => "VOTING_PRIVACY",
            Command::VotingSpamIssue => "VOTING_SPAM_ISSUE",
            Command::VotingSpamReject => "VOTING_SPAM_REJECT",
            Command::VotingStartDecay => "VOTING_START_DECAY",
            Command::VotingSuspendRounds => "VOTING_SUSPEND_ROUNDS",
            Command::VotingTimeout => "VOTING_TIMEOUT",
            Command::VotingTimeoutPerVoter => "VOTING_TIMEOUT_PER_VOTER",
            Command::WaitForExternalScript => "WAIT_FOR_EXTERNAL_SCRIPT",
            Command::WaitForExternalScriptTimeout => "WAIT_FOR_EXTERNAL_SCRIPT_TIMEOUT",
            Command::WallsLength => "WALLS_LENGTH",
            Command::WallsStayUpDelay => "WALLS_STAY_UP_DELAY",
            Command::WhitelistEnemiesIp => "WHITELIST_ENEMIES_IP",
            Command::WhitelistEnemiesUsername => "WHITELIST_ENEMIES_USERNAME",
            Command::WhiteSparks => "WHITE_SPARKS",
            Command::WinZoneDeaths => "WIN_ZONE_DEATHS",
            Command::WinZoneExpansion => "WIN_ZONE_EXPANSION",
            Command::WinZoneInitialSize => "WIN_ZONE_INITIAL_SIZE",
            Command::WinZoneMinLastDeath => "WIN_ZONE_MIN_LAST_DEATH",
            Command::WinZoneMinRoundTime => "WIN_ZONE_MIN_ROUND_TIME",
            Command::WinZoneRandomness => "WIN_ZONE_RANDOMNESS",
            Command::WordDelimiters => "WORD_DELIMITERS",
            Command::WrapMenu => "WRAP_MENU",
            Command::Zdepth => "ZDEPTH",
            Command::ZoneAlpha => "ZONE_ALPHA",
            Command::ZoneAlphaServer => "ZONE_ALPHA_SERVER",
            Command::ZoneAlphaToggle => "ZONE_ALPHA_TOGGLE",
            Command::ZoneBottom => "ZONE_BOTTOM",
            Command::ZoneHeight => "ZONE_HEIGHT",
            Command::ZoneSegments => "ZONE_SEGMENTS",
            Command::ZoneSegLength => "ZONE_SEG_LENGTH",
            #[cfg(feature = "ap")]
            Command::AccelzoneRate => "ACCELZONE_RATE",
            #[cfg(feature = "ap")]
            Command::AccessLevelAdmin => "ACCESS_LEVEL_ADMIN",
            #[cfg(feature = "ap")]
            Command::AccessLevelOp => "ACCESS_LEVEL_OP",
            #[cfg(feature = "ap")]
            Command::AccessLevelOpMax => "ACCESS_LEVEL_OP_MAX",
            #[cfg(feature = "ap")]
            Command::AccessLevelOpMin => "ACCESS_LEVEL_OP_MIN",
            #[cfg(feature = "ap")]
            Command::AccessLevelQueueConfigs => "ACCESS_LEVEL_QUEUE_CONFIGS",
            #[cfg(feature = "ap")]
            Command::AccessLevelQueueMaps => "ACCESS_LEVEL_QUEUE_MAPS",
            #[cfg(feature = "ap")]
            Command::AccessLevelReportsClear => "ACCESS_LEVEL_REPORTS_CLEAR",
            #[cfg(feature = "ap")]
            Command::AccessLevelReportsRead => "ACCESS_LEVEL_REPORTS_READ",
            #[cfg(feature = "ap")]
            Command::AccessLevelSpyConsole => "ACCESS_LEVEL_SPY_CONSOLE",
            #[cfg(feature = "ap")]
            Command::AccessLevelSubstitute => "ACCESS_LEVEL_SUBSTITUTE",
            #[cfg(feature = "ap")]
            Command::AccessLevelTeam => "ACCESS_LEVEL_TEAM",
            #[cfg(feature = "ap")]
            Command::AccessLevelViewChats => "ACCESS_LEVEL_VIEW_CHATS",
            #[cfg(feature = "ap")]
            Command::AddScorePlayer => "ADD_SCORE_PLAYER",
            #[cfg(feature = "ap")]
            Command::AddScoreTeam => "ADD_SCORE_TEAM",
            #[cfg(feature = "ap")]
            Command::AddZoneIdRoute => "ADD_ZONE_ID_ROUTE",
            #[cfg(feature = "ap")]
            Command::AddZoneRoute => "ADD_ZONE_ROUTE",
            #[cfg(feature = "ap")]
            Command::AdminKillMessage => "ADMIN_KILL_MESSAGE",
            #[cfg(feature = "ap")]
            Command::AdminLog => "ADMIN_LOG",
            #[cfg(feature = "ap")]
            Command::AdminName => "ADMIN_NAME",
            #[cfg(feature = "ap")]
            Command::AiBypass => "AI_BYPASS",
            #[cfg(feature = "ap")]
            Command::AiReload => "AI_RELOAD",
            #[cfg(feature = "ap")]
            Command::AllowTeamNameLeader => "ALLOW_TEAM_NAME_LEADER",
            #[cfg(feature = "ap")]
            Command::Announce => "ANNOUNCE",
            #[cfg(feature = "ap")]
            Command::ApplyRotation => "APPLY_ROTATION",
            #[cfg(feature = "ap")]
            Command::ApplySpecForce => "APPLY_SPEC_FORCE",
            #[cfg(feature = "ap")]
            Command::ApplyTeamForce => "APPLY_TEAM_FORCE",
            #[cfg(feature = "ap")]
            Command::ArenaBoundary => "ARENA_BOUNDARY",
            #[cfg(feature = "ap")]
            Command::ArenaBoundaryKill => "ARENA_BOUNDARY_KILL",
            #[cfg(feature = "ap")]
            Command::AutoSubstitution => "AUTO_SUBSTITUTION",
            #[cfg(feature = "ap")]
            Command::BallsInteract => "BALLS_INTERACT",
            #[cfg(feature = "ap")]
            Command::BallAutorespawn => "BALL_AUTORESPAWN",
            #[cfg(feature = "ap")]
            Command::BallCycleAccelBoost => "BALL_CYCLE_ACCEL_BOOST",
            #[cfg(feature = "ap")]
            Command::BallKills => "BALL_KILLS",
            #[cfg(feature = "ap")]
            Command::BallSpeedDecay => "BALL_SPEED_DECAY",
            #[cfg(feature = "ap")]
            Command::BallSpeedHitDecay => "BALL_SPEED_HIT_DECAY",
            #[cfg(feature = "ap")]
            Command::BallTeamMode => "BALL_TEAM_MODE",
            #[cfg(feature = "ap")]
            Command::BannedWords => "BANNED_WORDS",
            #[cfg(feature = "ap")]
            Command::BannedWordsAdd => "BANNED_WORDS_ADD",
            #[cfg(feature = "ap")]
            Command::BannedWordsDelimiters => "BANNED_WORDS_DELIMITERS",
            #[cfg(feature = "ap")]
            Command::BannedWordsList => "BANNED_WORDS_LIST",
            #[cfg(feature = "ap")]
            Command::BannedWordsOptions => "BANNED_WORDS_OPTIONS",
            #[cfg(feature = "ap")]
            Command::BannedWordsRemove => "BANNED_WORDS_REMOVE",
            #[cfg(feature = "ap")]
            Command::BannedWordsWhole => "BANNED_WORDS_WHOLE",
            #[cfg(feature = "ap")]
            Command::BaseEnemyKill => "BASE_ENEMY_KILL",
            #[cfg(feature = "ap")]
            Command::BaseEnemyRespawn => "BASE_ENEMY_RESPAWN",
            #[cfg(feature = "ap")]
            Command::BaseRespawn => "BASE_RESPAWN",
            #[cfg(feature = "ap")]
            Command::BaseRespawnRemindTime => "BASE_RESPAWN_REMIND_TIME",
            #[cfg(feature = "ap")]
            Command::Boot => "BOOT",
            #[cfg(feature = "ap")]
            Command::CameraForbidMer => "CAMERA_FORBID_MER",
            #[cfg(feature = "ap")]
            Command::CameraGlanceForwardSnap => "CAMERA_GLANCE_FORWARD_SNAP",
            #[cfg(feature = "ap")]
            Command::CameraGlanceHold => "CAMERA_GLANCE_HOLD",
            #[cfg(feature = "ap")]
            Command::CameraGlanceSnap => "CAMERA_GLANCE_SNAP",
            #[cfg(feature = "ap")]
            Command::CenterPlayerMessage => "CENTER_PLAYER_MESSAGE",
            #[cfg(feature = "ap")]
            Command::CfgUserSave => "CFG_USER_SAVE",
            #[cfg(feature = "ap")]
            Command::ChatbotControlledByServer => "CHATBOT_CONTROLLED_BY_SERVER",
            #[cfg(feature = "ap")]
            Command::ChatbotEnabled => "CHATBOT_ENABLED",
            #[cfg(feature = "ap")]
            Command::ChatlogWriteTeam => "CHATLOG_WRITE_TEAM",
            #[cfg(feature = "ap")]
            Command::ChattersKill => "CHATTERS_KILL",
            #[cfg(feature = "ap")]
            Command::ChattersList => "CHATTERS_LIST",
            #[cfg(feature = "ap")]
            Command::ChattersSilence => "CHATTERS_SILENCE",
            #[cfg(feature = "ap")]
            Command::ChattersSuspend => "CHATTERS_SUSPEND",
            #[cfg(feature = "ap")]
            Command::ChatLogColors => "CHAT_LOG_COLORS",
            #[cfg(feature = "ap")]
            Command::ClearChatlog => "CLEAR_CHATLOG",
            #[cfg(feature = "ap")]
            Command::ClearLadderlog => "CLEAR_LADDERLOG",
            #[cfg(feature = "ap")]
            Command::ClearReports => "CLEAR_REPORTS",
            #[cfg(feature = "ap")]
            Command::ClearScorelog => "CLEAR_SCORELOG",
            #[cfg(feature = "ap")]
            Command::ClientDownloadSettings => "CLIENT_DOWNLOAD_SETTINGS",
            #[cfg(feature = "ap")]
            Command::CollapseAll => "COLLAPSE_ALL",
            #[cfg(feature = "ap")]
            Command::CollapseZone => "COLLAPSE_ZONE",
            #[cfg(feature = "ap")]
            Command::CollapseZoneId => "COLLAPSE_ZONE_ID",
            #[cfg(feature = "ap")]
            Command::ColorDeathzoneBlue => "COLOR_DEATHZONE_BLUE",
            #[cfg(feature = "ap")]
            Command::ColorDeathzoneGreen => "COLOR_DEATHZONE_GREEN",
            #[cfg(feature = "ap")]
            Command::ColorDeathzoneRed => "COLOR_DEATHZONE_RED",
            #[cfg(feature = "ap")]
            Command::ColorRubberzoneBlue => "COLOR_RUBBERZONE_BLUE",
            #[cfg(feature = "ap")]
            Command::ColorRubberzoneGreen => "COLOR_RUBBERZONE_GREEN",
            #[cfg(feature = "ap")]
            Command::ColorRubberzoneRed => "COLOR_RUBBERZONE_RED",
            #[cfg(feature = "ap")]
            Command::ColorTeleportzoneBlue => "COLOR_TELEPORTZONE_BLUE",
            #[cfg(feature = "ap")]
            Command::ColorTeleportzoneGreen => "COLOR_TELEPORTZONE_GREEN",
            #[cfg(feature = "ap")]
            Command::ColorTeleportzoneRed => "COLOR_TELEPORTZONE_RED",
            #[cfg(feature = "ap")]
            Command::ColorWinzoneBlue => "COLOR_WINZONE_BLUE",
            #[cfg(feature = "ap")]
            Command::ColorWinzoneGreen => "COLOR_WINZONE_GREEN",
            #[cfg(feature = "ap")]
            Command::ColorWinzoneRed => "COLOR_WINZONE_RED",
            #[cfg(feature = "ap")]
            Command::CondenseConquestOutput => "CONDENSE_CONQUEST_OUTPUT",
            #[cfg(feature = "ap")]
            Command::ConfigRotation => "CONFIG_ROTATION",
            #[cfg(feature = "ap")]
            Command::ConfigRotationAdd => "CONFIG_ROTATION_ADD",
            #[cfg(feature = "ap")]
            Command::ConfigRotationLoad => "CONFIG_ROTATION_LOAD",
            #[cfg(feature = "ap")]
            Command::ConfigRotationRemove => "CONFIG_ROTATION_REMOVE",
            #[cfg(feature = "ap")]
            Command::ConfigRotationSet => "CONFIG_ROTATION_SET",
            #[cfg(feature = "ap")]
            Command::ConfigRotationType => "CONFIG_ROTATION_TYPE",
            #[cfg(feature = "ap")]
            Command::ConfigStorage => "CONFIG_STORAGE",
            #[cfg(feature = "ap")]
            Command::ConsoleDecorateId => "CONSOLE_DECORATE_ID",
            #[cfg(feature = "ap")]
            Command::ConsoleDecorateIp => "CONSOLE_DECORATE_IP",
            #[cfg(feature = "ap")]
            Command::ConsoleDecorateTimestamp => "CONSOLE_DECORATE_TIMESTAMP",
            #[cfg(feature = "ap")]
            Command::ConsoleLogColor => "CONSOLE_LOG_COLOR",
            #[cfg(feature = "ap")]
            Command::ConsoleLogColorDecorateTimestamp => "CONSOLE_LOG_COLOR_DECORATE_TIMESTAMP",
            #[cfg(feature = "ap")]
            Command::CustomAuthority => "CUSTOM_AUTHORITY",
            #[cfg(feature = "ap")]
            Command::CustomAuthorityConnection => "CUSTOM_AUTHORITY_CONNECTION",
            #[cfg(feature = "ap")]
            Command::CustomAuthorityEnabled => "CUSTOM_AUTHORITY_ENABLED",
            #[cfg(feature = "ap")]
            Command::CustomCenterMessage => "CUSTOM_CENTER_MESSAGE",
            #[cfg(feature = "ap")]
            Command::CustomCenterPlayerMessage => "CUSTOM_CENTER_PLAYER_MESSAGE",
            #[cfg(feature = "ap")]
            Command::CustomConfigs => "CUSTOM_CONFIGS",
            #[cfg(feature = "ap")]
            Command::CustomInvalidCommands => "CUSTOM_INVALID_COMMANDS",
            #[cfg(feature = "ap")]
            Command::CustomMessage => "CUSTOM_MESSAGE",
            #[cfg(feature = "ap")]
            Command::CustomPlayerCenterMessage => "CUSTOM_PLAYER_CENTER_MESSAGE",
            #[cfg(feature = "ap")]
            Command::CustomPlayerMessage => "CUSTOM_PLAYER_MESSAGE",
            #[cfg(feature = "ap")]
            Command::CustomShorthandAdd => "CUSTOM_SHORTHAND_ADD",
            #[cfg(feature = "ap")]
            Command::CustomShorthandAllowed => "CUSTOM_SHORTHAND_ALLOWED",
            #[cfg(feature = "ap")]
            Command::CustomShorthandEnabled => "CUSTOM_SHORTHAND_ENABLED",
            #[cfg(feature = "ap")]
            Command::CustomShorthandLinksList => "CUSTOM_SHORTHAND_LINKS_LIST",
            #[cfg(feature = "ap")]
            Command::CustomShorthandList => "CUSTOM_SHORTHAND_LIST",
            #[cfg(feature = "ap")]
            Command::CustomShorthandRemove => "CUSTOM_SHORTHAND_REMOVE",
            #[cfg(feature = "ap")]
            Command::CycleDeathTeleport => "CYCLE_DEATH_TELEPORT",
            #[cfg(feature = "ap")]
            Command::CycleDeathTeleportExplosion => "CYCLE_DEATH_TELEPORT_EXPLOSION",
            #[cfg(feature = "ap")]
            Command::CycleDeathTeleportReset => "CYCLE_DEATH_TELEPORT_RESET",
            #[cfg(feature = "ap")]
            Command::CycleDelayBonus => "CYCLE_DELAY_BONUS",
            #[cfg(feature = "ap")]
            Command::CycleRespawnZone => "CYCLE_RESPAWN_ZONE",
            #[cfg(feature = "ap")]
            Command::CycleRespawnZoneEnemy => "CYCLE_RESPAWN_ZONE_ENEMY",
            #[cfg(feature = "ap")]
            Command::CycleRespawnZoneEnemyKill => "CYCLE_RESPAWN_ZONE_ENEMY_KILL",
            #[cfg(feature = "ap")]
            Command::CycleRespawnZoneGrowth => "CYCLE_RESPAWN_ZONE_GROWTH",
            #[cfg(feature = "ap")]
            Command::CycleRespawnZoneRadius => "CYCLE_RESPAWN_ZONE_RADIUS",
            #[cfg(feature = "ap")]
            Command::CycleRespawnZoneRespawn => "CYCLE_RESPAWN_ZONE_RESPAWN",
            #[cfg(feature = "ap")]
            Command::CycleRespawnZoneType => "CYCLE_RESPAWN_ZONE_TYPE",
            #[cfg(feature = "ap")]
            Command::CycleRubberDepleteEnemy => "CYCLE_RUBBER_DEPLETE_ENEMY",
            #[cfg(feature = "ap")]
            Command::CycleRubberDepleteEnemyOverride => "CYCLE_RUBBER_DEPLETE_ENEMY_OVERRIDE",
            #[cfg(feature = "ap")]
            Command::CycleRubberDepleteRim => "CYCLE_RUBBER_DEPLETE_RIM",
            #[cfg(feature = "ap")]
            Command::CycleRubberDepleteRimOverride => "CYCLE_RUBBER_DEPLETE_RIM_OVERRIDE",
            #[cfg(feature = "ap")]
            Command::CycleRubberDepleteSelf => "CYCLE_RUBBER_DEPLETE_SELF",
            #[cfg(feature = "ap")]
            Command::CycleRubberDepleteSelfOverride => "CYCLE_RUBBER_DEPLETE_SELF_OVERRIDE",
            #[cfg(feature = "ap")]
            Command::CycleRubberDepleteTeam => "CYCLE_RUBBER_DEPLETE_TEAM",
            #[cfg(feature = "ap")]
            Command::CycleRubberDepleteTeamOverride => "CYCLE_RUBBER_DEPLETE_TEAM_OVERRIDE",
            #[cfg(feature = "ap")]
            Command::CycleTurn => "CYCLE_TURN",
            #[cfg(feature = "ap")]
            Command::CycleZonesAim => "CYCLE_ZONES_AIM",
            #[cfg(feature = "ap")]
            Command::CycleZonesAimChatbot => "CYCLE_ZONES_AIM_CHATBOT",
            #[cfg(feature = "ap")]
            Command::CycleZonesAimTypes => "CYCLE_ZONES_AIM_TYPES",
            #[cfg(feature = "ap")]
            Command::CycleZonesApproach => "CYCLE_ZONES_APPROACH",
            #[cfg(feature = "ap")]
            Command::CycleZonesApproch => "CYCLE_ZONES_APPROCH",
            #[cfg(feature = "ap")]
            Command::CycleZonesAvoid => "CYCLE_ZONES_AVOID",
            #[cfg(feature = "ap")]
            Command::CycleZonesAvoidAimOrder => "CYCLE_ZONES_AVOID_AIM_ORDER",
            #[cfg(feature = "ap")]
            Command::CycleZonesAvoidChatbot => "CYCLE_ZONES_AVOID_CHATBOT",
            #[cfg(feature = "ap")]
            Command::CycleZonesAvoidOld => "CYCLE_ZONES_AVOID_OLD",
            #[cfg(feature = "ap")]
            Command::CycleZonesAvoidRange => "CYCLE_ZONES_AVOID_RANGE",
            #[cfg(feature = "ap")]
            Command::DeadlyExplosions => "DEADLY_EXPLOSIONS",
            #[cfg(feature = "ap")]
            Command::DeathzoneRandomColors => "DEATHZONE_RANDOM_COLORS",
            #[cfg(feature = "ap")]
            Command::DeathzoneRotation => "DEATHZONE_ROTATION",
            #[cfg(feature = "ap")]
            Command::DeathzoneRotationSpeed => "DEATHZONE_ROTATION_SPEED",
            #[cfg(feature = "ap")]
            Command::DeathShot => "DEATH_SHOT",
            #[cfg(feature = "ap")]
            Command::DedicatedFps => "DEDICATED_FPS",
            #[cfg(feature = "ap")]
            Command::DedicatedFpsIdleFactor => "DEDICATED_FPS_IDLE_FACTOR",
            #[cfg(feature = "ap")]
            Command::DefaultMapFile => "DEFAULT_MAP_FILE",
            #[cfg(feature = "ap")]
            Command::DefaultMapFileOnEmpty => "DEFAULT_MAP_FILE_ON_EMPTY",
            #[cfg(feature = "ap")]
            Command::DelayCommand => "DELAY_COMMAND",
            #[cfg(feature = "ap")]
            Command::DelayCommandClear => "DELAY_COMMAND_CLEAR",
            #[cfg(feature = "ap")]
            Command::DelayCommandRemove => "DELAY_COMMAND_REMOVE",
            #[cfg(feature = "ap")]
            Command::Deop => "DEOP",
            #[cfg(feature = "ap")]
            Command::DestroyAll => "DESTROY_ALL",
            #[cfg(feature = "ap")]
            Command::DestroyZone => "DESTROY_ZONE",
            #[cfg(feature = "ap")]
            Command::DestroyZoneId => "DESTROY_ZONE_ID",
            #[cfg(feature = "ap")]
            Command::DisplayScoresDuringChat => "DISPLAY_SCORES_DURING_CHAT",
            #[cfg(feature = "ap")]
            Command::EnableFriendsCasing => "ENABLE_FRIENDS_CASING",
            #[cfg(feature = "ap")]
            Command::Exit => "EXIT",
            #[cfg(feature = "ap")]
            Command::FlagBlinkEnd => "FLAG_BLINK_END",
            #[cfg(feature = "ap")]
            Command::FlagBlinkEstimatePosition => "FLAG_BLINK_ESTIMATE_POSITION",
            #[cfg(feature = "ap")]
            Command::FlagBlinkOnTime => "FLAG_BLINK_ON_TIME",
            #[cfg(feature = "ap")]
            Command::FlagBlinkStart => "FLAG_BLINK_START",
            #[cfg(feature = "ap")]
            Command::FlagBlinkTime => "FLAG_BLINK_TIME",
            #[cfg(feature = "ap")]
            Command::FlagBlinkTrackTime => "FLAG_BLINK_TRACK_TIME",
            #[cfg(feature = "ap")]
            Command::FlagChatBlinkTime => "FLAG_CHAT_BLINK_TIME",
            #[cfg(feature = "ap")]
            Command::FlagColorB => "FLAG_COLOR_B",
            #[cfg(feature = "ap")]
            Command::FlagColorG => "FLAG_COLOR_G",
            #[cfg(feature = "ap")]
            Command::FlagColorR => "FLAG_COLOR_R",
            #[cfg(feature = "ap")]
            Command::FlagConquestWinsRound => "FLAG_CONQUEST_WINS_ROUND",
            #[cfg(feature = "ap")]
            Command::FlagControls => "FLAG_CONTROLS",
            #[cfg(feature = "ap")]
            Command::FlagDropHome => "FLAG_DROP_HOME",
            #[cfg(feature = "ap")]
            Command::FlagDropTime => "FLAG_DROP_TIME",
            #[cfg(feature = "ap")]
            Command::FlagHoldScore => "FLAG_HOLD_SCORE",
            #[cfg(feature = "ap")]
            Command::FlagHoldScoreTime => "FLAG_HOLD_SCORE_TIME",
            #[cfg(feature = "ap")]
            Command::FlagHoldTime => "FLAG_HOLD_TIME",
            #[cfg(feature = "ap")]
            Command::FlagHoldTimeDrop => "FLAG_HOLD_TIME_DROP",
            #[cfg(feature = "ap")]
            Command::FlagHomeRandomnessX => "FLAG_HOME_RANDOMNESS_X",
            #[cfg(feature = "ap")]
            Command::FlagHomeRandomnessY => "FLAG_HOME_RANDOMNESS_Y",
            #[cfg(feature = "ap")]
            Command::FlagPassDistance => "FLAG_PASS_DISTANCE",
            #[cfg(feature = "ap")]
            Command::FlagPassMode => "FLAG_PASS_MODE",
            #[cfg(feature = "ap")]
            Command::FlagPassSpeed => "FLAG_PASS_SPEED",
            #[cfg(feature = "ap")]
            Command::FlagRequiredHome => "FLAG_REQUIRED_HOME",
            #[cfg(feature = "ap")]
            Command::FlagTeam => "FLAG_TEAM",
            #[cfg(feature = "ap")]
            Command::ForbidHudMap => "FORBID_HUD_MAP",
            #[cfg(feature = "ap")]
            Command::ForceRespawnScript => "FORCE_RESPAWN_SCRIPT",
            #[cfg(feature = "ap")]
            Command::FullscreenPlayerMessage => "FULLSCREEN_PLAYER_MESSAGE",
            #[cfg(feature = "ap")]
            Command::GameSpHumans => "GAME_SP_HUMANS",
            #[cfg(feature = "ap")]
            Command::GameWaitPlayersEnabled => "GAME_WAIT_PLAYERS_ENABLED",
            #[cfg(feature = "ap")]
            Command::GetCurrentMap => "GET_CURRENT_MAP",
            #[cfg(feature = "ap")]
            Command::GivePoints => "GIVE_POINTS",
            #[cfg(feature = "ap")]
            Command::GlanceForwardTooltip => "GLANCE_FORWARD_TOOLTIP",
            #[cfg(feature = "ap")]
            Command::GoalRoundEnd => "GOAL_ROUND_END",
            #[cfg(feature = "ap")]
            Command::Help => "HELP",
            #[cfg(feature = "ap")]
            Command::HelpMessage => "HELP_MESSAGE",
            #[cfg(feature = "ap")]
            Command::HelpMessageType => "HELP_MESSAGE_TYPE",
            #[cfg(feature = "ap")]
            Command::IdleKickExempt => "IDLE_KICK_EXEMPT",
            #[cfg(feature = "ap")]
            Command::InterceptCommands => "INTERCEPT_COMMANDS",
            #[cfg(feature = "ap")]
            Command::InterceptUnknownCommands => "INTERCEPT_UNKNOWN_COMMANDS",
            #[cfg(feature = "ap")]
            Command::KillAll => "KILL_ALL",
            #[cfg(feature = "ap")]
            Command::KillAllScripts => "KILL_ALL_SCRIPTS",
            #[cfg(feature = "ap")]
            Command::KillId => "KILL_ID",
            #[cfg(feature = "ap")]
            Command::KillScript => "KILL_SCRIPT",
            #[cfg(feature = "ap")]
            Command::KohScore => "KOH_SCORE",
            #[cfg(feature = "ap")]
            Command::KohScoreTime => "KOH_SCORE_TIME",
            #[cfg(feature = "ap")]
            Command::LadderlogEnabled => "LADDERLOG_ENABLED",
            #[cfg(feature = "ap")]
            Command::LadderlogObjectzonePlayerEnteredInside => {
                "LADDERLOG_OBJECTZONE_PLAYER_ENTERED_INSIDE"
            }

            #[cfg(feature = "ap")]
            Command::LadderlogObjectzoneZoneEnteredPollrate => {
                "LADDERLOG_OBJECTZONE_ZONE_ENTERED_POLLRATE"
            }

            #[cfg(feature = "ap")]
            Command::LadderlogWriteAdminCommand => "LADDERLOG_WRITE_ADMIN_COMMAND",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteAdminLogin => "LADDERLOG_WRITE_ADMIN_LOGIN",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteAdminLogout => "LADDERLOG_WRITE_ADMIN_LOGOUT",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteAiPositions => "LADDERLOG_WRITE_AI_POSITIONS",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteBallVanish => "LADDERLOG_WRITE_BALL_VANISH",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteBasezoneConquererTeam => {
                "LADDERLOG_WRITE_BASEZONE_CONQUERER_TEAM"
            }

            #[cfg(feature = "ap")]
            Command::LadderlogWriteBaseEnemyRespawn => "LADDERLOG_WRITE_BASE_ENEMY_RESPAWN",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteBaseRespawn => "LADDERLOG_WRITE_BASE_RESPAWN",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteBlastzonePlayerEnter => "LADDERLOG_WRITE_BLASTZONE_PLAYER_ENTER",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteCommand => "LADDERLOG_WRITE_COMMAND",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteCurrentMap => "LADDERLOG_WRITE_CURRENT_MAP",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteCustomInvalidCommand => "LADDERLOG_WRITE_CUSTOM_INVALID_COMMAND",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteCycleCreated => "LADDERLOG_WRITE_CYCLE_CREATED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteCycleDeathTeleport => "LADDERLOG_WRITE_CYCLE_DEATH_TELEPORT",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteCycleDestroyed => "LADDERLOG_WRITE_CYCLE_DESTROYED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteDeathzoneActivated => "LADDERLOG_WRITE_DEATHZONE_ACTIVATED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteDeathBasezoneConquered => {
                "LADDERLOG_WRITE_DEATH_BASEZONE_CONQUERED"
            }

            #[cfg(feature = "ap")]
            Command::LadderlogWriteDeathDeathshot => "LADDERLOG_WRITE_DEATH_DEATHSHOT",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteDeathDeathzone => "LADDERLOG_WRITE_DEATH_DEATHZONE",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteDeathDeathzoneTeam => "LADDERLOG_WRITE_DEATH_DEATHZONE_TEAM",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteDeathExplosion => "LADDERLOG_WRITE_DEATH_EXPLOSION",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteDeathRubberzone => "LADDERLOG_WRITE_DEATH_RUBBERZONE",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteDeathSelfDestruct => "LADDERLOG_WRITE_DEATH_SELF_DESTRUCT",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteDeathShotFrag => "LADDERLOG_WRITE_DEATH_SHOT_FRAG",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteDeathShotSuicide => "LADDERLOG_WRITE_DEATH_SHOT_SUICIDE",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteDeathShotTeamkill => "LADDERLOG_WRITE_DEATH_SHOT_TEAMKILL",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteDeathZombiezone => "LADDERLOG_WRITE_DEATH_ZOMBIEZONE",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteEndChallenge => "LADDERLOG_WRITE_END_CHALLENGE",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteFlagConquestRoundWin => {
                "LADDERLOG_WRITE_FLAG_CONQUEST_ROUND_WIN"
            }

            #[cfg(feature = "ap")]
            Command::LadderlogWriteFlagDrop => "LADDERLOG_WRITE_FLAG_DROP",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteFlagHeld => "LADDERLOG_WRITE_FLAG_HELD",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteFlagReturn => "LADDERLOG_WRITE_FLAG_RETURN",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteFlagScore => "LADDERLOG_WRITE_FLAG_SCORE",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteFlagTake => "LADDERLOG_WRITE_FLAG_TAKE",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteFlagTimeout => "LADDERLOG_WRITE_FLAG_TIMEOUT",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteInvalidCommand => "LADDERLOG_WRITE_INVALID_COMMAND",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteMatchEnded => "LADDERLOG_WRITE_MATCH_ENDED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteMatchScore => "LADDERLOG_WRITE_MATCH_SCORE",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteMatchScoreTeam => "LADDERLOG_WRITE_MATCH_SCORE_TEAM",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteNewSet => "LADDERLOG_WRITE_NEW_SET",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteNextRound => "LADDERLOG_WRITE_NEXT_ROUND",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteObjectzonePlayerEntered => {
                "LADDERLOG_WRITE_OBJECTZONE_PLAYER_ENTERED"
            }
            #[cfg(feature = "ap")]
            Command::LadderlogWriteObjectzonePlayerLeft => "LADDERLOG_WRITE_OBJECTZONE_PLAYER_LEFT",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteObjectzoneSpawned => "LADDERLOG_WRITE_OBJECTZONE_SPAWNED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteObjectzoneZoneEntered => {
                "LADDERLOG_WRITE_OBJECTZONE_ZONE_ENTERED"
            }
            #[cfg(feature = "ap")]
            Command::LadderlogWriteObjectzoneZoneLeft => "LADDERLOG_WRITE_OBJECTZONE_ZONE_LEFT",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteOnlineAi => "LADDERLOG_WRITE_ONLINE_AI",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteOnlinePlayersAlive => "LADDERLOG_WRITE_ONLINE_PLAYERS_ALIVE",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteOnlinePlayersCount => "LADDERLOG_WRITE_ONLINE_PLAYERS_COUNT",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteOnlinePlayersDead => "LADDERLOG_WRITE_ONLINE_PLAYERS_DEAD",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteOnlineTeam => "LADDERLOG_WRITE_ONLINE_TEAM",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteOnlineZone => "LADDERLOG_WRITE_ONLINE_ZONE",
            #[cfg(feature = "ap")]
            Command::LadderlogWritePlayerAiEntered => "LADDERLOG_WRITE_PLAYER_AI_ENTERED",
            #[cfg(feature = "ap")]
            Command::LadderlogWritePlayerAiLeft => "LADDERLOG_WRITE_PLAYER_AI_LEFT",
            #[cfg(feature = "ap")]
            Command::LadderlogWritePlayerColoredName => "LADDERLOG_WRITE_PLAYER_COLORED_NAME",
            #[cfg(feature = "ap")]
            Command::LadderlogWritePlayerEnteredGrid => "LADDERLOG_WRITE_PLAYER_ENTERED_GRID",
            #[cfg(feature = "ap")]
            Command::LadderlogWritePlayerEnteredSpectator => {
                "LADDERLOG_WRITE_PLAYER_ENTERED_SPECTATOR"
            }
            #[cfg(feature = "ap")]
            Command::LadderlogWritePlayerGridpos => "LADDERLOG_WRITE_PLAYER_GRIDPOS",
            #[cfg(feature = "ap")]
            Command::LadderlogWritePlayerJoinsSpectators => {
                "LADDERLOG_WRITE_PLAYER_JOINS_SPECTATORS"
            }
            #[cfg(feature = "ap")]
            Command::LadderlogWritePlayerKilled => "LADDERLOG_WRITE_PLAYER_KILLED",
            #[cfg(feature = "ap")]
            Command::LadderlogWritePlayerLeavesSpectators => {
                "LADDERLOG_WRITE_PLAYER_LEAVES_SPECTATORS"
            }
            #[cfg(feature = "ap")]
            Command::LadderlogWritePlayerLogin => "LADDERLOG_WRITE_PLAYER_LOGIN",
            #[cfg(feature = "ap")]
            Command::LadderlogWritePlayerLogout => "LADDERLOG_WRITE_PLAYER_LOGOUT",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteQueueFinished => "LADDERLOG_WRITE_QUEUE_FINISHED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteQueueStarted => "LADDERLOG_WRITE_QUEUE_STARTED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteRoundCommencing => "LADDERLOG_WRITE_ROUND_COMMENCING",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteRoundEnded => "LADDERLOG_WRITE_ROUND_ENDED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteRoundFinished => "LADDERLOG_WRITE_ROUND_FINISHED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteRoundStarted => "LADDERLOG_WRITE_ROUND_STARTED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteSetWinner => "LADDERLOG_WRITE_SET_WINNER",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteShutdown => "LADDERLOG_WRITE_SHUTDOWN",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteSoccerBallPlayerEntered => {
                "LADDERLOG_WRITE_SOCCER_BALL_PLAYER_ENTERED"
            }
            #[cfg(feature = "ap")]
            Command::LadderlogWriteSoccerGoalPlayerEntered => {
                "LADDERLOG_WRITE_SOCCER_GOAL_PLAYER_ENTERED"
            }
            #[cfg(feature = "ap")]
            Command::LadderlogWriteSoccerGoalScored => "LADDERLOG_WRITE_SOCCER_GOAL_SCORED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteSpawnPositionTeam => "LADDERLOG_WRITE_SPAWN_POSITION_TEAM",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteStartChallenge => "LADDERLOG_WRITE_START_CHALLENGE",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteSvgCreated => "LADDERLOG_WRITE_SVG_CREATED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteTacticalPosition => "LADDERLOG_WRITE_TACTICAL_POSITION",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteTacticalStatistics => "LADDERLOG_WRITE_TACTICAL_STATISTICS",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteTargetzoneConquered => "LADDERLOG_WRITE_TARGETZONE_CONQUERED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteTargetzonePlayerEnter => {
                "LADDERLOG_WRITE_TARGETZONE_PLAYER_ENTER"
            }
            #[cfg(feature = "ap")]
            Command::LadderlogWriteTargetzonePlayerLeft => "LADDERLOG_WRITE_TARGETZONE_PLAYER_LEFT",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteTargetzoneTimeout => "LADDERLOG_WRITE_TARGETZONE_TIMEOUT",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteTeamColoredName => "LADDERLOG_WRITE_TEAM_COLORED_NAME",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteVoter => "LADDERLOG_WRITE_VOTER",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteVoteCreated => "LADDERLOG_WRITE_VOTE_CREATED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteWinzoneActivated => "LADDERLOG_WRITE_WINZONE_ACTIVATED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteWinzonePlayerEnter => "LADDERLOG_WRITE_WINZONE_PLAYER_ENTER",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteZoneCollapsed => "LADDERLOG_WRITE_ZONE_COLLAPSED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteZoneCreated => "LADDERLOG_WRITE_ZONE_CREATED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteZoneGridpos => "LADDERLOG_WRITE_ZONE_GRIDPOS",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteZoneRouteStopped => "LADDERLOG_WRITE_ZONE_ROUTE_STOPPED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteZoneShotReleased => "LADDERLOG_WRITE_ZONE_SHOT_RELEASED",
            #[cfg(feature = "ap")]
            Command::LadderlogWriteZoneSpawned => "LADDERLOG_WRITE_ZONE_SPAWNED",
            #[cfg(feature = "ap")]
            Command::LadderHighscoreOutput => "LADDER_HIGHSCORE_OUTPUT",
            #[cfg(feature = "ap")]
            Command::LanguageReload => "LANGUAGE_RELOAD",
            #[cfg(feature = "ap")]
            Command::LegacyLadderlogCommand => "LEGACY_LADDERLOG_COMMAND",
            #[cfg(feature = "ap")]
            Command::LimitAdvance => "LIMIT_ADVANCE",
            #[cfg(feature = "ap")]
            Command::LimitScoreMinLead => "LIMIT_SCORE_MIN_LEAD",
            #[cfg(feature = "ap")]
            Command::LimitSets => "LIMIT_SETS",
            #[cfg(feature = "ap")]
            Command::ListAllCommands => "LIST_ALL_COMMANDS",
            #[cfg(feature = "ap")]
            Command::ListAllCommandsLevels => "LIST_ALL_COMMANDS_LEVELS",
            #[cfg(feature = "ap")]
            Command::ListScripts => "LIST_SCRIPTS",
            #[cfg(feature = "ap")]
            Command::LoadCustomConfigs => "LOAD_CUSTOM_CONFIGS",
            #[cfg(feature = "ap")]
            Command::Login => "LOGIN",
            #[cfg(feature = "ap")]
            Command::Logout => "LOGOUT",
            #[cfg(feature = "ap")]
            Command::LogPlayersActivities => "LOG_PLAYERS_ACTIVITIES",
            #[cfg(feature = "ap")]
            Command::LogTurns => "LOG_TURNS",
            #[cfg(feature = "ap")]
            Command::LogTurnsTimestamp => "LOG_TURNS_TIMESTAMP",
            #[cfg(feature = "ap")]
            Command::LogTurnsWinner => "LOG_TURNS_WINNER",
            #[cfg(feature = "ap")]
            Command::LogZoneGridpos => "LOG_ZONE_GRIDPOS",
            #[cfg(feature = "ap")]
            Command::LogZoneGridposId => "LOG_ZONE_GRIDPOS_ID",
            #[cfg(feature = "ap")]
            Command::MapOnchangeInclude => "MAP_ONCHANGE_INCLUDE",
            #[cfg(feature = "ap")]
            Command::MapRotation => "MAP_ROTATION",
            #[cfg(feature = "ap")]
            Command::MapRotationAdd => "MAP_ROTATION_ADD",
            #[cfg(feature = "ap")]
            Command::MapRotationLoad => "MAP_ROTATION_LOAD",
            #[cfg(feature = "ap")]
            Command::MapRotationRemove => "MAP_ROTATION_REMOVE",
            #[cfg(feature = "ap")]
            Command::MapRotationSet => "MAP_ROTATION_SET",
            #[cfg(feature = "ap")]
            Command::MapStorage => "MAP_STORAGE",
            #[cfg(feature = "ap")]
            Command::MegaShotDir => "MEGA_SHOT_DIR",
            #[cfg(feature = "ap")]
            Command::MegaShotExplosion => "MEGA_SHOT_EXPLOSION",
            #[cfg(feature = "ap")]
            Command::MegaShotMult => "MEGA_SHOT_MULT",
            #[cfg(feature = "ap")]
            Command::MegaShotThresh => "MEGA_SHOT_THRESH",
            #[cfg(feature = "ap")]
            Command::MinFlagsHome => "MIN_FLAGS_HOME",
            #[cfg(feature = "ap")]
            Command::MoveHere => "MOVE_HERE",
            #[cfg(feature = "ap")]
            Command::NumAisPerRound => "NUM_AIS_PER_ROUND",
            #[cfg(feature = "ap")]
            Command::OnlineStatsInterval => "ONLINE_STATS_INTERVAL",
            #[cfg(feature = "ap")]
            Command::Op => "OP",
            #[cfg(feature = "ap")]
            Command::PlayerCenterMessage => "PLAYER_CENTER_MESSAGE",
            #[cfg(feature = "ap")]
            Command::PlayerFullscreenMessage => "PLAYER_FULLSCREEN_MESSAGE",
            #[cfg(feature = "ap")]
            Command::PlayerGridposInterval => "PLAYER_GRIDPOS_INTERVAL",
            #[cfg(feature = "ap")]
            Command::PlayerGridposOnTurn => "PLAYER_GRIDPOS_ON_TURN",
            #[cfg(feature = "ap")]
            Command::PlayerUniqueColor => "PLAYER_UNIQUE_COLOR",
            #[cfg(feature = "ap")]
            Command::PortMax => "PORT_MAX",
            #[cfg(feature = "ap")]
            Command::PortMin => "PORT_MIN",
            #[cfg(feature = "ap")]
            Command::PredictWalls => "PREDICT_WALLS",
            #[cfg(feature = "ap")]
            Command::QueuersList => "QUEUERS_LIST",
            #[cfg(feature = "ap")]
            Command::QueueConfig => "QUEUE_CONFIG",
            #[cfg(feature = "ap")]
            Command::QueueEnabled => "QUEUE_ENABLED",
            #[cfg(feature = "ap")]
            Command::QueueGive => "QUEUE_GIVE",
            #[cfg(feature = "ap")]
            Command::QueueIncrement => "QUEUE_INCREMENT",
            #[cfg(feature = "ap")]
            Command::QueueLimit => "QUEUE_LIMIT",
            #[cfg(feature = "ap")]
            Command::QueueLimitEnabled => "QUEUE_LIMIT_ENABLED",
            #[cfg(feature = "ap")]
            Command::QueueLimitExcempt => "QUEUE_LIMIT_EXCEMPT",
            #[cfg(feature = "ap")]
            Command::QueueLog => "QUEUE_LOG",
            #[cfg(feature = "ap")]
            Command::QueueMap => "QUEUE_MAP",
            #[cfg(feature = "ap")]
            Command::QueueMax => "QUEUE_MAX",
            #[cfg(feature = "ap")]
            Command::QueueRefill => "QUEUE_REFILL",
            #[cfg(feature = "ap")]
            Command::QueueRefillActive => "QUEUE_REFILL_ACTIVE",
            #[cfg(feature = "ap")]
            Command::QueueRefillTime => "QUEUE_REFILL_TIME",
            #[cfg(feature = "ap")]
            Command::Quit => "QUIT",
            #[cfg(feature = "ap")]
            Command::RaceChances => "RACE_CHANCES",
            #[cfg(feature = "ap")]
            Command::RaceCheckpointCountdown => "RACE_CHECKPOINT_COUNTDOWN",
            #[cfg(feature = "ap")]
            Command::RaceCheckpointLaps => "RACE_CHECKPOINT_LAPS",
            #[cfg(feature = "ap")]
            Command::RaceCheckpointRequireHit => "RACE_CHECKPOINT_REQUIRE_HIT",
            #[cfg(feature = "ap")]
            Command::RaceEndDelay => "RACE_END_DELAY",
            #[cfg(feature = "ap")]
            Command::RaceFinishCollapse => "RACE_FINISH_COLLAPSE",
            #[cfg(feature = "ap")]
            Command::RaceFinishKill => "RACE_FINISH_KILL",
            #[cfg(feature = "ap")]
            Command::RaceIdleKill => "RACE_IDLE_KILL",
            #[cfg(feature = "ap")]
            Command::RaceIdleSpeed => "RACE_IDLE_SPEED",
            #[cfg(feature = "ap")]
            Command::RaceIdleTime => "RACE_IDLE_TIME",
            #[cfg(feature = "ap")]
            Command::RaceIdleWarnings => "RACE_IDLE_WARNINGS",
            #[cfg(feature = "ap")]
            Command::RaceLaps => "RACE_LAPS",
            #[cfg(feature = "ap")]
            Command::RaceLogLogin => "RACE_LOG_LOGIN",
            #[cfg(feature = "ap")]
            Command::RaceLogTime => "RACE_LOG_TIME",
            #[cfg(feature = "ap")]
            Command::RaceLogUnfinished => "RACE_LOG_UNFINISHED",
            #[cfg(feature = "ap")]
            Command::RaceNumRanksShowEnd => "RACE_NUM_RANKS_SHOW_END",
            #[cfg(feature = "ap")]
            Command::RaceNumRanksShowStart => "RACE_NUM_RANKS_SHOW_START",
            #[cfg(feature = "ap")]
            Command::RacePointsType => "RACE_POINTS_TYPE",
            #[cfg(feature = "ap")]
            Command::RaceRanksShowEnd => "RACE_RANKS_SHOW_END",
            #[cfg(feature = "ap")]
            Command::RaceRanksShowStart => "RACE_RANKS_SHOW_START",
            #[cfg(feature = "ap")]
            Command::RaceRankHeaderLength => "RACE_RANK_HEADER_LENGTH",
            #[cfg(feature = "ap")]
            Command::RaceRankHeaderOrder => "RACE_RANK_HEADER_ORDER",
            #[cfg(feature = "ap")]
            Command::RaceRankHeaderPlayerLength => "RACE_RANK_HEADER_PLAYER_LENGTH",
            #[cfg(feature = "ap")]
            Command::RaceRankHeaderPlayerOrder => "RACE_RANK_HEADER_PLAYER_ORDER",
            #[cfg(feature = "ap")]
            Command::RaceRankHeaderTimeLength => "RACE_RANK_HEADER_TIME_LENGTH",
            #[cfg(feature = "ap")]
            Command::RaceRankHeaderTimeOrder => "RACE_RANK_HEADER_TIME_ORDER",
            #[cfg(feature = "ap")]
            Command::RaceRankShowLength => "RACE_RANK_SHOW_LENGTH",
            #[cfg(feature = "ap")]
            Command::RaceRankShowPlayerLength => "RACE_RANK_SHOW_PLAYER_LENGTH",
            #[cfg(feature = "ap")]
            Command::RaceRecordsLoad => "RACE_RECORDS_LOAD",
            #[cfg(feature = "ap")]
            Command::RaceRecordsSave => "RACE_RECORDS_SAVE",
            #[cfg(feature = "ap")]
            Command::RaceSafeAngles => "RACE_SAFE_ANGLES",
            #[cfg(feature = "ap")]
            Command::RaceScoreDeplete => "RACE_SCORE_DEPLETE",
            #[cfg(feature = "ap")]
            Command::RaceSmartTimer => "RACE_SMART_TIMER",
            #[cfg(feature = "ap")]
            Command::RaceSmartTimerFactor => "RACE_SMART_TIMER_FACTOR",
            #[cfg(feature = "ap")]
            Command::RaceSmartTimerNum => "RACE_SMART_TIMER_NUM",
            #[cfg(feature = "ap")]
            Command::RaceTimerEnabled => "RACE_TIMER_ENABLED",
            #[cfg(feature = "ap")]
            Command::RaceUnsafeAnglesKill => "RACE_UNSAFE_ANGLES_KILL",
            #[cfg(feature = "ap")]
            Command::ReloadConfig => "RELOAD_CONFIG",
            #[cfg(feature = "ap")]
            Command::ResetConfigQueueing => "RESET_CONFIG_QUEUEING",
            #[cfg(feature = "ap")]
            Command::ResetMapQueueing => "RESET_MAP_QUEUEING",
            #[cfg(feature = "ap")]
            Command::ResetRotation => "RESET_ROTATION",
            #[cfg(feature = "ap")]
            Command::ResetRotationOnStartNewMatch => "RESET_ROTATION_ON_START_NEW_MATCH",
            #[cfg(feature = "ap")]
            Command::Respawn => "RESPAWN",
            #[cfg(feature = "ap")]
            Command::RespawnAll => "RESPAWN_ALL",
            #[cfg(feature = "ap")]
            Command::RespawnDefaultPosition => "RESPAWN_DEFAULT_POSITION",
            #[cfg(feature = "ap")]
            Command::RespawnMessage => "RESPAWN_MESSAGE",
            #[cfg(feature = "ap")]
            Command::RespawnPlayer => "RESPAWN_PLAYER",
            #[cfg(feature = "ap")]
            Command::RespawnScript => "RESPAWN_SCRIPT",
            #[cfg(feature = "ap")]
            Command::RespawnStrict => "RESPAWN_STRICT",
            #[cfg(feature = "ap")]
            Command::RespawnTime => "RESPAWN_TIME",
            #[cfg(feature = "ap")]
            Command::RevertMapFile => "REVERT_MAP_FILE",
            #[cfg(feature = "ap")]
            Command::RotationMax => "ROTATION_MAX",
            #[cfg(feature = "ap")]
            Command::RotationMaxType => "ROTATION_MAX_TYPE",
            #[cfg(feature = "ap")]
            Command::RotationMessage => "ROTATION_MESSAGE",
            #[cfg(feature = "ap")]
            Command::RotationType => "ROTATION_TYPE",
            #[cfg(feature = "ap")]
            Command::RubberzoneRate => "RUBBERZONE_RATE",
            #[cfg(feature = "ap")]
            Command::ScoreBlastzone => "SCORE_BLASTZONE",
            #[cfg(feature = "ap")]
            Command::ScoreDeathzoneTeam => "SCORE_DEATHZONE_TEAM",
            #[cfg(feature = "ap")]
            Command::ScoreDeathShot => "SCORE_DEATH_SHOT",
            #[cfg(feature = "ap")]
            Command::ScoreDiffWin => "SCORE_DIFF_WIN",
            #[cfg(feature = "ap")]
            Command::ScoreExplosion => "SCORE_EXPLOSION",
            #[cfg(feature = "ap")]
            Command::ScoreExplosionOwner => "SCORE_EXPLOSION_OWNER",
            #[cfg(feature = "ap")]
            Command::ScoreFlag => "SCORE_FLAG",
            #[cfg(feature = "ap")]
            Command::ScoreFlagHomeBase => "SCORE_FLAG_HOME_BASE",
            #[cfg(feature = "ap")]
            Command::ScoreGoal => "SCORE_GOAL",
            #[cfg(feature = "ap")]
            Command::ScoreRace => "SCORE_RACE",
            #[cfg(feature = "ap")]
            Command::ScoreRaceFinish => "SCORE_RACE_FINISH",
            #[cfg(feature = "ap")]
            Command::ScoreRubberzone => "SCORE_RUBBERZONE",
            #[cfg(feature = "ap")]
            Command::ScoreSelfDestruct => "SCORE_SELF_DESTRUCT",
            #[cfg(feature = "ap")]
            Command::ScoreShot => "SCORE_SHOT",
            #[cfg(feature = "ap")]
            Command::ScoreShotBase => "SCORE_SHOT_BASE",
            #[cfg(feature = "ap")]
            Command::ScoreShotSuicide => "SCORE_SHOT_SUICIDE",
            #[cfg(feature = "ap")]
            Command::ScoreZombieZone => "SCORE_ZOMBIE_ZONE",
            #[cfg(feature = "ap")]
            Command::ScoreZombieZoneRevenge => "SCORE_ZOMBIE_ZONE_REVENGE",
            #[cfg(feature = "ap")]
            Command::ScriptEnv => "SCRIPT_ENV",
            #[cfg(feature = "ap")]
            Command::SelfDestruct => "SELF_DESTRUCT",
            #[cfg(feature = "ap")]
            Command::SelfDestructFall => "SELF_DESTRUCT_FALL",
            #[cfg(feature = "ap")]
            Command::SelfDestructRadius => "SELF_DESTRUCT_RADIUS",
            #[cfg(feature = "ap")]
            Command::SelfDestructRise => "SELF_DESTRUCT_RISE",
            #[cfg(feature = "ap")]
            Command::SelfDestructRot => "SELF_DESTRUCT_ROT",
            #[cfg(feature = "ap")]
            Command::SelfDestructVanish => "SELF_DESTRUCT_VANISH",
            #[cfg(feature = "ap")]
            Command::ServerOptions => "SERVER_OPTIONS",
            #[cfg(feature = "ap")]
            Command::SetAiPosition => "SET_AI_POSITION",
            #[cfg(feature = "ap")]
            Command::SetCommandsAccesslevel => "SET_COMMANDS_ACCESSLEVEL",
            #[cfg(feature = "ap")]
            Command::SetCycleBraking => "SET_CYCLE_BRAKING",
            #[cfg(feature = "ap")]
            Command::SetCycleRubber => "SET_CYCLE_RUBBER",
            #[cfg(feature = "ap")]
            Command::SetCycleSpeed => "SET_CYCLE_SPEED",
            #[cfg(feature = "ap")]
            Command::SetPlayerTeam => "SET_PLAYER_TEAM",
            #[cfg(feature = "ap")]
            Command::SetTargetCommand => "SET_TARGET_COMMAND",
            #[cfg(feature = "ap")]
            Command::SetZoneColor => "SET_ZONE_COLOR",
            #[cfg(feature = "ap")]
            Command::SetZoneCoord => "SET_ZONE_COORD",
            #[cfg(feature = "ap")]
            Command::SetZoneDir => "SET_ZONE_DIR",
            #[cfg(feature = "ap")]
            Command::SetZoneExpansion => "SET_ZONE_EXPANSION",
            #[cfg(feature = "ap")]
            Command::SetZoneIdColor => "SET_ZONE_ID_COLOR",
            #[cfg(feature = "ap")]
            Command::SetZoneIdCoord => "SET_ZONE_ID_COORD",
            #[cfg(feature = "ap")]
            Command::SetZoneIdDir => "SET_ZONE_ID_DIR",
            #[cfg(feature = "ap")]
            Command::SetZoneIdExpansion => "SET_ZONE_ID_EXPANSION",
            #[cfg(feature = "ap")]
            Command::SetZoneIdPenetrate => "SET_ZONE_ID_PENETRATE",
            #[cfg(feature = "ap")]
            Command::SetZoneIdRadius => "SET_ZONE_ID_RADIUS",
            #[cfg(feature = "ap")]
            Command::SetZoneIdRotation => "SET_ZONE_ID_ROTATION",
            #[cfg(feature = "ap")]
            Command::SetZoneIdRoute => "SET_ZONE_ID_ROUTE",
            #[cfg(feature = "ap")]
            Command::SetZoneIdSpeed => "SET_ZONE_ID_SPEED",
            #[cfg(feature = "ap")]
            Command::SetZonePenetrate => "SET_ZONE_PENETRATE",
            #[cfg(feature = "ap")]
            Command::SetZoneRadius => "SET_ZONE_RADIUS",
            #[cfg(feature = "ap")]
            Command::SetZoneRotation => "SET_ZONE_ROTATION",
            #[cfg(feature = "ap")]
            Command::SetZoneRoute => "SET_ZONE_ROUTE",
            #[cfg(feature = "ap")]
            Command::SetZoneSpeed => "SET_ZONE_SPEED",
            #[cfg(feature = "ap")]
            Command::ShotBaseEnemyRespawn => "SHOT_BASE_ENEMY_RESPAWN",
            #[cfg(feature = "ap")]
            Command::ShotBaseRespawn => "SHOT_BASE_RESPAWN",
            #[cfg(feature = "ap")]
            Command::ShotCollision => "SHOT_COLLISION",
            #[cfg(feature = "ap")]
            Command::ShotDiscardTime => "SHOT_DISCARD_TIME",
            #[cfg(feature = "ap")]
            Command::ShotExplosion => "SHOT_EXPLOSION",
            #[cfg(feature = "ap")]
            Command::ShotKillEnemies => "SHOT_KILL_ENEMIES",
            #[cfg(feature = "ap")]
            Command::ShotKillInvulnerable => "SHOT_KILL_INVULNERABLE",
            #[cfg(feature = "ap")]
            Command::ShotKillSelf => "SHOT_KILL_SELF",
            #[cfg(feature = "ap")]
            Command::ShotKillVanish => "SHOT_KILL_VANISH",
            #[cfg(feature = "ap")]
            Command::ShotPenetrateWalls => "SHOT_PENETRATE_WALLS",
            #[cfg(feature = "ap")]
            Command::ShotRadiusMax => "SHOT_RADIUS_MAX",
            #[cfg(feature = "ap")]
            Command::ShotRadiusMin => "SHOT_RADIUS_MIN",
            #[cfg(feature = "ap")]
            Command::ShotRotMax => "SHOT_ROT_MAX",
            #[cfg(feature = "ap")]
            Command::ShotRotMin => "SHOT_ROT_MIN",
            #[cfg(feature = "ap")]
            Command::ShotSeekUpdateTime => "SHOT_SEEK_UPDATE_TIME",
            #[cfg(feature = "ap")]
            Command::ShotStartDist => "SHOT_START_DIST",
            #[cfg(feature = "ap")]
            Command::ShotThresh => "SHOT_THRESH",
            #[cfg(feature = "ap")]
            Command::ShotVelocityMult => "SHOT_VELOCITY_MULT",
            #[cfg(feature = "ap")]
            Command::ShotWallBounce => "SHOT_WALL_BOUNCE",
            #[cfg(feature = "ap")]
            Command::ShowMapAxes => "SHOW_MAP_AXES",
            #[cfg(feature = "ap")]
            Command::ShowMapCreation => "SHOW_MAP_CREATION",
            #[cfg(feature = "ap")]
            Command::ShowMapDetails => "SHOW_MAP_DETAILS",
            #[cfg(feature = "ap")]
            Command::ShufflePlayer => "SHUFFLE_PLAYER",
            #[cfg(feature = "ap")]
            Command::Shutdown => "SHUTDOWN",
            #[cfg(feature = "ap")]
            Command::ShutdownStop => "SHUTDOWN_STOP",
            #[cfg(feature = "ap")]
            Command::ShutdownTimeout => "SHUTDOWN_TIMEOUT",
            #[cfg(feature = "ap")]
            Command::SilenceAll => "SILENCE_ALL",
            #[cfg(feature = "ap")]
            Command::SilenceDead => "SILENCE_DEAD",
            #[cfg(feature = "ap")]
            Command::SilenceEnemies => "SILENCE_ENEMIES",
            #[cfg(feature = "ap")]
            Command::SoccerBallFirstWin => "SOCCER_BALL_FIRST_WIN",
            #[cfg(feature = "ap")]
            Command::SoccerBallScoreOwnGoal => "SOCCER_BALL_SCORE_OWN_GOAL",
            #[cfg(feature = "ap")]
            Command::SoccerBallShotsWin => "SOCCER_BALL_SHOTS_WIN",
            #[cfg(feature = "ap")]
            Command::SoccerBallSlowdown => "SOCCER_BALL_SLOWDOWN",
            #[cfg(feature = "ap")]
            Command::SoccerBallSlowdownHackymethod => "SOCCER_BALL_SLOWDOWN_HACKYMETHOD",
            #[cfg(feature = "ap")]
            Command::SoccerBallSlowdownSpeed => "SOCCER_BALL_SLOWDOWN_SPEED",
            #[cfg(feature = "ap")]
            Command::SoccerBallSlowdownSyncInterval => "SOCCER_BALL_SLOWDOWN_SYNC_INTERVAL",
            #[cfg(feature = "ap")]
            Command::SoccerGoalKillEnemies => "SOCCER_GOAL_KILL_ENEMIES",
            #[cfg(feature = "ap")]
            Command::SoccerGoalRespawnAllies => "SOCCER_GOAL_RESPAWN_ALLIES",
            #[cfg(feature = "ap")]
            Command::SoccerGoalRespawnEnemies => "SOCCER_GOAL_RESPAWN_ENEMIES",
            #[cfg(feature = "ap")]
            Command::SoccerGoalScore => "SOCCER_GOAL_SCORE",
            #[cfg(feature = "ap")]
            Command::SpawnAlternate => "SPAWN_ALTERNATE",
            #[cfg(feature = "ap")]
            Command::SpawnEnabled => "SPAWN_ENABLED",
            #[cfg(feature = "ap")]
            Command::SpawnExplosion => "SPAWN_EXPLOSION",
            #[cfg(feature = "ap")]
            Command::SpawnObjectzone => "SPAWN_OBJECTZONE",
            #[cfg(feature = "ap")]
            Command::SpawnScript => "SPAWN_SCRIPT",
            #[cfg(feature = "ap")]
            Command::SpawnSoccer => "SPAWN_SOCCER",
            #[cfg(feature = "ap")]
            Command::SpawnWinnersFirst => "SPAWN_WINNERS_FIRST",
            #[cfg(feature = "ap")]
            Command::SpawnWrap => "SPAWN_WRAP",
            #[cfg(feature = "ap")]
            Command::SpawnZone => "SPAWN_ZONE",
            #[cfg(feature = "ap")]
            Command::SpeakAsAdmin => "SPEAK_AS_ADMIN",
            #[cfg(feature = "ap")]
            Command::SpeakToEnemies => "SPEAK_TO_ENEMIES",
            #[cfg(feature = "ap")]
            Command::SpeakToEveryone => "SPEAK_TO_EVERYONE",
            #[cfg(feature = "ap")]
            Command::SpLimitAdvance => "SP_LIMIT_ADVANCE",
            #[cfg(feature = "ap")]
            Command::SpLimitScoreMinLead => "SP_LIMIT_SCORE_MIN_LEAD",
            #[cfg(feature = "ap")]
            Command::SpLimitSets => "SP_LIMIT_SETS",
            #[cfg(feature = "ap")]
            Command::SpScoreDiffWin => "SP_SCORE_DIFF_WIN",
            #[cfg(feature = "ap")]
            Command::StyctCompatibilityLadderlogPlayerGridpos => {
                "STYCT_COMPATIBILITY_LADDERLOG_PLAYER_GRIDPOS"
            }
            #[cfg(feature = "ap")]
            Command::StyctCompatibilitySetZoneColor => "STYCT_COMPATIBILITY_SET_ZONE_COLOR",
            #[cfg(feature = "ap")]
            Command::StyctCompatibilitySpawnZone => "STYCT_COMPATIBILITY_SPAWN_ZONE",
            #[cfg(feature = "ap")]
            Command::SuicideMessage => "SUICIDE_MESSAGE",
            #[cfg(feature = "ap")]
            Command::SuspendAll => "SUSPEND_ALL",
            #[cfg(feature = "ap")]
            Command::SuspendList => "SUSPEND_LIST",
            #[cfg(feature = "ap")]
            Command::SvgZoneRotationAnimate => "SVG_ZONE_ROTATION_ANIMATE",
            #[cfg(feature = "ap")]
            Command::SwapWinzoneDeathzoneColors => "SWAP_WINZONE_DEATHZONE_COLORS",
            #[cfg(feature = "ap")]
            Command::TakePoints => "TAKE_POINTS",
            #[cfg(feature = "ap")]
            Command::TargetzoneColorB => "TARGETZONE_COLOR_B",
            #[cfg(feature = "ap")]
            Command::TargetzoneColorG => "TARGETZONE_COLOR_G",
            #[cfg(feature = "ap")]
            Command::TargetzoneColorR => "TARGETZONE_COLOR_R",
            #[cfg(feature = "ap")]
            Command::TargetDeclareWinner => "TARGET_DECLARE_WINNER",
            #[cfg(feature = "ap")]
            Command::TargetInitialScore => "TARGET_INITIAL_SCORE",
            #[cfg(feature = "ap")]
            Command::TargetLifetime => "TARGET_LIFETIME",
            #[cfg(feature = "ap")]
            Command::TargetPlayerMultiuse => "TARGET_PLAYER_MULTIUSE",
            #[cfg(feature = "ap")]
            Command::TargetScoreDeplete => "TARGET_SCORE_DEPLETE",
            #[cfg(feature = "ap")]
            Command::TargetSurviveTime => "TARGET_SURVIVE_TIME",
            #[cfg(feature = "ap")]
            Command::Teleport => "TELEPORT",
            #[cfg(feature = "ap")]
            Command::TeleportPlayer => "TELEPORT_PLAYER",
            #[cfg(feature = "ap")]
            Command::TextBrightBackground => "TEXT_BRIGHT_BACKGROUND",
            #[cfg(feature = "ap")]
            Command::TextShadow => "TEXT_SHADOW",
            #[cfg(feature = "ap")]
            Command::TimerMax => "TIMER_MAX",
            #[cfg(feature = "ap")]
            Command::TimerMin => "TIMER_MIN",
            #[cfg(feature = "ap")]
            Command::TimerMode => "TIMER_MODE",
            #[cfg(feature = "ap")]
            Command::TimerReset => "TIMER_RESET",
            #[cfg(feature = "ap")]
            Command::TimerResume => "TIMER_RESUME",
            #[cfg(feature = "ap")]
            Command::TimerStart => "TIMER_START",
            #[cfg(feature = "ap")]
            Command::TimerStop => "TIMER_STOP",
            #[cfg(feature = "ap")]
            Command::TimerType => "TIMER_TYPE",
            #[cfg(feature = "ap")]
            Command::UnsilenceAll => "UNSILENCE_ALL",
            #[cfg(feature = "ap")]
            Command::UnsuspendAll => "UNSUSPEND_ALL",
            #[cfg(feature = "ap")]
            Command::VerifyColorStrict => "VERIFY_COLOR_STRICT",
            #[cfg(feature = "ap")]
            Command::VoiceAll => "VOICE_ALL",
            #[cfg(feature = "ap")]
            Command::VotingBiasChallenge => "VOTING_BIAS_CHALLENGE",
            #[cfg(feature = "ap")]
            Command::WinzonePlayerEnterWin => "WINZONE_PLAYER_ENTER_WIN",
            #[cfg(feature = "ap")]
            Command::ZombieZone => "ZOMBIE_ZONE",
            #[cfg(feature = "ap")]
            Command::ZombieZoneFall => "ZOMBIE_ZONE_FALL",
            #[cfg(feature = "ap")]
            Command::ZombieZoneRadius => "ZOMBIE_ZONE_RADIUS",
            #[cfg(feature = "ap")]
            Command::ZombieZoneRise => "ZOMBIE_ZONE_RISE",
            #[cfg(feature = "ap")]
            Command::ZombieZoneRot => "ZOMBIE_ZONE_ROT",
            #[cfg(feature = "ap")]
            Command::ZombieZoneShoot => "ZOMBIE_ZONE_SHOOT",
            #[cfg(feature = "ap")]
            Command::ZombieZoneSpeed => "ZOMBIE_ZONE_SPEED",
            #[cfg(feature = "ap")]
            Command::ZombieZoneVanish => "ZOMBIE_ZONE_VANISH",
            #[cfg(feature = "ap")]
            Command::ZonesBounceOnCycleWalls => "ZONES_BOUNCE_ON_CYCLE_WALLS",
            #[cfg(feature = "ap")]
            Command::ZoneDelayClear => "ZONE_DELAY_CLEAR",
            #[cfg(feature = "ap")]
            Command::ZoneGridposInterval => "ZONE_GRIDPOS_INTERVAL",
            #[cfg(feature = "ap")]
            Command::ZoneHeightFort => "ZONE_HEIGHT_FORT",
            #[cfg(feature = "ap")]
            Command::ZoneHeightKoh => "ZONE_HEIGHT_KOH",
            #[cfg(feature = "ap")]
            Command::ZoneNoFadeInServer => "ZONE_NO_FADE_IN_SERVER",
            #[cfg(feature = "ap")]
            Command::ZoneSegSteps => "ZONE_SEG_STEPS",
            #[cfg(feature = "ap")]
            Command::ZoneSpeedDecay => "ZONE_SPEED_DECAY",
            #[cfg(feature = "ap")]
            Command::ZoneSpeedHitDecay => "ZONE_SPEED_HIT_DECAY",
            #[cfg(feature = "ap")]
            Command::ZoneWallBoundary => "ZONE_WALL_BOUNDARY",
            #[cfg(feature = "ap")]
            Command::ZoneWallBoundaryValueRestricted => "ZONE_WALL_BOUNDARY_VALUE_RESTRICTED",
            #[cfg(feature = "ap")]
            Command::ZoneWallDeath => "ZONE_WALL_DEATH",
        };
        write!(f, "{}", word)
    }
}
