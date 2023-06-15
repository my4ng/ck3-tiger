#[derive(Copy, Clone, Debug, Eq, PartialEq, Display, EnumString)]
pub enum Datatype {
    Unknown,
    AIUnitWatchWindow,
    AIWarCoordinatorWatchWindow,
    AIWatchWindow,
    Accolade,
    AccoladeHistoryEntry,
    AccoladeRank,
    AccoladeType,
    AccoladeView,
    Achievement,
    AchievementPopup,
    AchievementWindow,
    ActionItemHandler,
    ActiveActivityOption,
    ActiveCasusBelli,
    ActiveCouncilTask,
    ActiveCouncilTaskIcon,
    Activity,
    ActivityGuestCustomData,
    ActivityGuestInviteRule,
    ActivityGuestListWindow,
    ActivityIntent,
    ActivityIntentData,
    ActivityIntentSelectionWindow,
    ActivityListDetailHostView,
    ActivityListDetailJoinView,
    ActivityListWindow,
    ActivityLocale,
    ActivityLocaleWindow,
    ActivityLogEntry,
    ActivityLogWindow,
    ActivityOption,
    ActivityOptionCategory,
    ActivityPhase,
    ActivityPlanner,
    ActivityPlannerMapIcon,
    ActivityPulseAction,
    ActivityPulseEffect,
    ActivitySpecialGuest,
    ActivitySpecialGuestType,
    ActivityType,
    ActivityWindow,
    ActivityWindowBackgroundData,
    ActivityWindowCharacter,
    AddFriendWindow,
    AddTraditionWindow,
    AllianceInfo,
    AllyListSubview,
    AnchorItem,
    AnimationTestGroupItem,
    AnimationTestWindow,
    Application,
    AppointCourtPositionView,
    Army,
    ArmyComposition,
    ArmyRegiment,
    ArmyReorgWindow,
    ArmyWindow,
    Artifact,
    ArtifactClaim,
    ArtifactClaimsList,
    ArtifactDetailsView,
    ArtifactHelperWindow,
    ArtifactHistory,
    ArtifactKillListWindow,
    ArtifactSettings,
    ArtifactType,
    ArtifactVisualType,
    AssetSettings,
    AttachToArmyWindow,
    Attribute,
    AvailabilityEntry,
    BarbershopAccessoryCategory,
    BarbershopAccessoryItem,
    BarbershopBackgroundCategory,
    BarbershopBackgroundItem,
    BarbershopCameraSettings,
    BarbershopCategory,
    BarbershopCharacter,
    BarbershopCoAHandler,
    BarbershopColorPickerHandler,
    BarbershopItem,
    BarbershopPhotoPreset,
    BarbershopPortrait,
    BarbershopPoseDropDown,
    BarbershopPoseItem,
    BarbershopPoseSettings,
    BarbershopScreenshotHandler,
    BarbershopWindow,
    BattleEvent,
    BattleSummaryWindow,
    BlackmailInteractionWindow,
    BlackmailSecretItem,
    BlurThreshold,
    Bookmark,
    BookmarkCharacter,
    BookmarkCharacterGUI,
    BookmarkGroup,
    BookmarkGroupItem,
    BookmarkItem,
    BookmarkPortrait,
    BottomBarSchemeItem,
    BrushBool,
    BrushFloat,
    BrushSettings,
    BrushSettingsDropdown,
    BrushSettingsGlobal,
    Building,
    BuildingLevelItem,
    CActivityWindowBase,
    CCourtLanguageMapIcon,
    CFixedPoint,
    CPdxEnumValue,
    CPdxFloatRect,
    CPdxInputBindingSetting,
    CPdxIntRect,
    CSelectCommanderWindow,
    CString,
    CUTF8String,
    CVector2f,
    CVector2i,
    CVector3f,
    CVector3i,
    CVector4f,
    CVector4i,
    CallAllyInteractionNotificationWindow,
    CallAllyInteractionWindow,
    CallAllyWarItem,
    Camera,
    CameraItem,
    CantCreateOrJoinFactionVassal,
    CapitalMapIcon,
    CasusBelliItem,
    CasusBelliTitleItem,
    CasusBelliType,
    CatalystEntry,
    CatalystHistory,
    CatalystType,
    ChangeGHWTargetWindow,
    ChangeGHWTargetWindowTitleItem,
    Character,
    CharacterFilterPreset,
    CharacterFinderWindow,
    CharacterFocusWindow,
    CharacterInteraction,
    CharacterInteractionCategory,
    CharacterInteractionConfirmationWindow,
    CharacterInteractionMenuWindow,
    CharacterInteractionNotificationWindow,
    CharacterItem,
    CharacterLifestyleWindow,
    CharacterListFilter,
    CharacterListFilterCategory,
    CharacterListFilterOption,
    CharacterListItem,
    CharacterListSortItem,
    CharacterMemory,
    CharacterMemoryType,
    CharacterPoolWatchWindow,
    CharacterProperties,
    CharacterSelectionList,
    CharacterWindow,
    Chat,
    ChatMessage,
    ChatNotificationMessage,
    ChatTab,
    ChatWindow,
    ChildGenerator,
    ChildItem,
    Claim,
    ClaimTitleItem,
    ClaimantEntry,
    ClaimantSortOption,
    ClaimantsWindow,
    CloudSaveData,
    CoatOfArms,
    CoatOfArmsDesigner,
    CoatOfArmsDesignerBackgroundPanel,
    CoatOfArmsDesignerEmblemInstance,
    CoatOfArmsDesignerEmblemInstancesPanel,
    CoatOfArmsDesignerEmblemLayout,
    CoatOfArmsDesignerEmblemLayoutPanel,
    CoatOfArmsDesignerEmblemTexture,
    CoatOfArmsDesignerPaletteColor,
    CoatOfArmsDesignerPattern,
    CollapsibleCultureList,
    CollapsibleCultureListGroup,
    CollapsibleReligionList,
    CollapsibleReligionListGroup,
    ColorGenePicker,
    Combat,
    CombatEffect,
    CombatMaaItem,
    CombatMapIcon,
    CombatPredictionEdge,
    CombatPredictionMapIcon,
    CombatResultData,
    CombatRollModifiers,
    CombatSide,
    CombatSideModifierItem,
    CombatSideResultData,
    CombatWindow,
    ComplexBar,
    ComplexBarItem,
    ConcubineInfo,
    ConcubineInteractionWindow,
    ConsoleMenuItem,
    ConsoleWindow,
    Container,
    ContextMenuItem,
    CostBreakdown,
    CouncilPositionType,
    CouncilTaskInteractionItem,
    CouncilTaskInteractionWindow,
    CouncilTaskType,
    CouncilWindow,
    CountryEntry,
    County,
    CountyGroup,
    CourtAmenitiesCategoryItem,
    CourtAmenitiesSetting,
    CourtAmenitiesSettingItem,
    CourtAmenitiesWindow,
    CourtEventItem,
    CourtEventWindow,
    CourtGrandeurData,
    CourtGrandeurLevel,
    CourtGrandeurWindow,
    CourtPosition,
    CourtPositionCategory,
    CourtPositionType,
    CourtPositionsWindow,
    CourtSceneEditorWindow,
    CourtSceneSettings,
    CourtToolset,
    CourtType,
    CourtTypeSettingItem,
    CourtTypeWindow,
    CourtWindow,
    CreateAccoladeView,
    CreateClaimantFactionAgainstWindow,
    CreateFactionItem,
    CreateSocialProfile,
    CreateSocialProfileWindow,
    CreditPortraitData,
    CreditsWindow,
    Culture,
    CultureAesthetics,
    CultureEra,
    CultureEraType,
    CultureInnovation,
    CultureInnovationType,
    CulturePillar,
    CultureReformation,
    CultureTemplate,
    CultureTradition,
    CultureWindow,
    CurveEditor,
    CurvePoint,
    DatatypesExplorer,
    Date,
    DeJureVassalGroupItem,
    DebugTutorialChainItem,
    DebugTutorialLessonItem,
    DebugTutorialStepItem,
    DebugTutorialWindow,
    Decision,
    DecisionDetailView,
    DecisionViewWidget,
    DecisionViewWidgetCreateHolyOrder,
    DecisionViewWidgetOptionList,
    DecisionsView,
    DecisionsViewItem,
    DeclareWarInteractionWindow,
    DesignateHeirWindow,
    DesignerCoA,
    DetailData,
    DetailedConfirmationView,
    Diarchy,
    DiarchySuccessor,
    DiarchyWindow,
    DiplomacyItem,
    DiplomaticItem,
    DisplayedInteractionEffects,
    DivergenceWindow,
    Dlc,
    DlcCollection,
    DlcInfoGui,
    DlcItem,
    DockableLayout,
    DockableLayoutManager,
    DockableWindow,
    DoctrineGroupWindow,
    DoctrineGroupingFetcher,
    DoctrineGroupingFetcher2,
    DrawCmdsList,
    DrawCmdsViewer,
    DuchyGroup,
    Dynasty,
    DynastyCustomizationWindow,
    DynastyHouse,
    DynastyHouseIcon,
    DynastyHouseMembersWindow,
    DynastyHouseTemplate,
    DynastyHouseView,
    DynastyLegacy,
    DynastyLegacyItem,
    DynastyPerk,
    DynastyPerkConfirmation,
    DynastyTemplate,
    DynastyTreeItem,
    DynastyTreeView,
    DynastyView,
    EmitterNodeWindow,
    EmployedPositionItem,
    EmptyCourtPosition,
    Encyclopedia,
    EncyclopediaEntry,
    EncyclopediaEntryView,
    EncyclopediaPage,
    EndPrepConfirm,
    EntityEditor,
    EntityEditorAudioEventHandler,
    EntityEditorEventLayer,
    EntityEditorKeyframe,
    EntityEditorTimelineState,
    EntityViewerProperties,
    Entry,
    EnumSettingEntry,
    ErrorMessageBox,
    Ethnicity,
    EthnicityItem,
    EventChainProgressEntry,
    EventInfo,
    EventLayerForEntityEditor,
    EventOption,
    EventTargetSetupContext,
    EventWindow,
    EventWindowBackgroundData,
    EventWindowCustomWidgetStruggleInfo,
    EventWindowData,
    EventWindowTransitionData,
    EventWindowViewInsert,
    EventWindowWidget,
    EventWindowWidgetChainProgress,
    EventWindowWidgetEnterText,
    EventWindowWidgetEnterTextDefaultEntry,
    EventWindowWidgetNameCharacter,
    ExportTool,
    EyeDropper,
    EyeDropperPackedSample,
    Faction,
    FactionCharacterMember,
    FactionCountyMember,
    FactionItem,
    FactionsWindow,
    Faith,
    FaithConversionWindow,
    FaithCreationWindow,
    FaithDoctrine,
    FaithDoctrineGroup,
    FaithWindow,
    FeedMessageItem,
    FilterPresetItem,
    FindTitleView,
    FindVassalListWindow,
    Fleet,
    FleetPredictionMapIcon,
    Focus,
    FocusType,
    Friend,
    FriendListWindow,
    FriendRequest,
    FriendSearchResult,
    Friends,
    FrontEndCreditsView,
    FrontEndLoadView,
    FrontEndMainView,
    FrontEndMultiplayerView,
    FrontEndView,
    GUIAchievement,
    GUIAlertItem,
    GUIBuildingItem,
    GUICountyHolding,
    GUIPotentialBuildingItem,
    GUITrackItem,
    GameConceptTooltip,
    GameDialog,
    GameMpSetup,
    GameRule,
    GameRuleCategory,
    GameRuleSetting,
    GameSetup,
    GeneCategory,
    GeneItem,
    GeneTemplate,
    GenerationItem,
    GeographicalRegion,
    GfxSkin,
    GovernmentType,
    GovernmentTypeFilter,
    GrantTitleOffer,
    GrantTitlesInteractionWindow,
    Graph,
    GraphInterfaceNodeWindow,
    GraphPanel,
    GreatHolyWar,
    GreatHolyWarParticipant,
    GreatHolyWarParticipantScore,
    GreatHolyWarWindow,
    Group,
    GuiAIWarCoordinator,
    GuiActionImportantActionItem,
    GuiActionItem,
    GuiAnimationCurveEditor,
    GuiAnimationCurveEditorControlPoint,
    GuiAnimationCurveEditorLine,
    GuiAnimationCurveEditorViewport,
    GuiAnimationEditor,
    GuiAnimationEditorAnimSetEntry,
    GuiAnimationEditorAnimationEntry,
    GuiAnimationEditorAvailableTrack,
    GuiAnimationEditorKeyframe,
    GuiAnimationEditorMetadataCtx,
    GuiAnimationEditorPlayer,
    GuiAnimationEditorPlayerSpeedMultiplierEntry,
    GuiAnimationEditorRuler,
    GuiAnimationEditorRulerResolutionEntry,
    GuiAnimationEditorUniversalTrack,
    GuiAnimationEditorViewportBase,
    GuiAnimationEditorViewportUserInput,
    GuiAnimationTimelineViewport,
    GuiClaimant,
    GuiContext,
    GuiCouncilPosition,
    GuiCultureEra,
    GuiCultureEraGroup,
    GuiCultureInnovation,
    GuiCultureTradition,
    GuiEditor,
    GuiEditorCategory,
    GuiEditorDockable,
    GuiEditorOutliner,
    GuiEditorProperties,
    GuiEditorProperty,
    GuiEditorTooltip,
    GuiFaithCreationDoctrineItem,
    GuiFaithDoctrineItem,
    GuiFaithIcon,
    GuiGameRule,
    GuiGameRulePreset,
    GuiHolySiteItem,
    GuiLaw,
    GuiLawGroup,
    GuiMilitaryStrength,
    GuiPotentialCouncilTask,
    GuiUnitInfo,
    GuiVirtueOrSinItem,
    Heightmap,
    HeightmapPainter,
    HeightmapPainterMode,
    HeightmapResolution,
    HiredTroop,
    HiredTroopDetailView,
    HiredTroopItem,
    HiredTroopRegiment,
    HistoryEntry,
    Holding,
    HoldingItem,
    HoldingType,
    HoldingTypeItem,
    HoldingView,
    HolyOrder,
    HolySite,
    HolySiteIcon,
    Hook,
    HouseCustomizationWindow,
    HouseOrderOption,
    HudBottomWidget,
    HybridizationWindow,
    Illustration,
    ImagePopup,
    ImportTool,
    Importable,
    ImportableGroup,
    ImportantActionItem,
    ImportantActionType,
    InFrontTopBar,
    InGameBottomBar,
    InGameTopbar,
    InfoboxNodeWindow,
    IngameWindow,
    InputActionBinding,
    InspectorPanel,
    Inspiration,
    InspirationType,
    InspirationsWindow,
    InteractionCategoryItem,
    InteractionContainer,
    InteractionEffectsDescription,
    InteractionItem,
    InteractionOtherEffect,
    InteractionParticipantsEffect,
    InteractionSchemeInfo,
    InteractionTitleItem,
    InterfereInWarInteractionNotificationWindow,
    InterfereInWarInteractionWindow,
    InterfereInWarWarItem,
    IntrigueWindow,
    IntrigueWindowHookItem,
    IntrigueWindowSecretGroup,
    IntrigueWindowSecretItem,
    Inventory,
    InventorySlot,
    InventorySlotType,
    InventoryView,
    InviteCreateClaimantFactionOffer,
    JominiGUISetting,
    JominiGameRules,
    JominiNotification,
    JominiNotificationOverlay,
    JominiPasswordPopup,
    JominiServer,
    JominiServerBrowserGui,
    JominiSettingsWindow,
    KeyframeEditor,
    KeyframeEventEditor,
    KeyframeWidget,
    KillListWindow,
    KnightsView,
    LandedTitpleTemplate,
    LanguageEntry,
    LanguageWindow,
    Law,
    LawGroup,
    LawItem,
    Layer,
    LayerTreeItem,
    LayeredIcon,
    LeaseOutBaroniesWindow,
    LeaseOutBaroniesWindowTitleItem,
    LegacyItem,
    LevyView,
    Lifestyle,
    Light,
    LineOfSuccessionItem,
    LoadIngameWindow,
    LobbyPlayer,
    LobbyView,
    LocalPlayerActivityData,
    LocalPlayerCachedData,
    LocalPlayerCourtEvents,
    LocalPlayerNewArtifacts,
    LogEntry,
    LogViewer,
    LogViewerCategory,
    LogViewerEntry,
    LogViewerType,
    MAAOriginMapIcon,
    MPConfig,
    Mandate,
    MapContentEditorMode,
    MapContentEditorOptions,
    MapContentEditorViewport,
    MapContentEntryDesc,
    MapContentLayerDesc,
    MapContentPanel,
    MapContentPropertyGroup,
    MapContentPropertyGroupsGui,
    MapContentSelector,
    MapContentSelectorGui,
    MapEditor,
    MapEditorGui,
    MapEditorLayerBorder,
    MapEditorLayerBorderDockable,
    MapMode,
    MapObjectMask,
    MapObjectPainter,
    MapObjectPainterMode,
    MapObjectPainterOptions,
    MapObjectTool,
    MarriageInfo,
    MarriageInteractionNotificationWindow,
    MarriageInteractionWindow,
    MarriageOffer,
    MaskEntry,
    MaskManagerEntry,
    MaskPainterManager,
    MaskPainterMapContentPanel,
    MaskPainterMode,
    MaskPainterTool,
    MaskPainterViewport,
    MatchOffer,
    MatchmakerInteractionWindow,
    MatchmakerTraitInfo,
    Material,
    MaterialBrowser,
    MaterialEntry,
    MaterialMix,
    MaterialMixBrush,
    MaterialMixEntry,
    MaterialPaintingMode,
    Materials,
    MaterialsSample,
    MemoriesWindow,
    MemoryInfo,
    MenAtArmsType,
    MenAtArmsTypeView,
    MenAtArmsTypeViewTypeItem,
    MenAtArmsView,
    MercenaryCompany,
    MessageFeedHandler,
    MessageType,
    MetaInfoWidget,
    MilitaryItem,
    MilitaryView,
    MilitaryViewEventTroop,
    MixBrushMode,
    ModifierItem,
    ModifyVassalContractInteractionWindow,
    ModifyVassalContractInteractionWindowObligationLevelOption,
    MoveTool,
    MpBookmarkItem,
    MultiplayerSetupWindow,
    MusicPlayerCategory,
    MusicTrack,
    MyRealmWindow,
    MyRealmWindowVassalItem,
    Nickname,
    Node,
    NodeEditorSearch,
    NodeError,
    NodeLine,
    NodePin,
    NodeWindow,
    NonRegisteredDockable,
    NotificationDummyContext,
    Nudger,
    NudgerLayerEntryMapObjectDesc,
    NudgerMapContentGui,
    NudgerMapObjectPropertyListDockable,
    NudgerMode,
    ObjectBrowser,
    ObjectBrowserView,
    ObjectInspector,
    ObjectInspectorDockable,
    ObjectInspectorPlugin,
    ObjectPreset,
    ObjectProvider,
    ObligationContainerData,
    ObligationLevel,
    ObligationLevelCheckbox,
    ObligationLevelLineConnection,
    ObligationLevelLineItem,
    ObligationLevelLineTree,
    ObligationLevelRadioButtons,
    OosData,
    OosWindow,
    OptionEffectItem,
    OptionItem,
    OrderFaithOption,
    OrderedActivityInviteRule,
    OutgoingFriendRequest,
    Outliner,
    OutlinerHoldingItem,
    OutlinerPlayer,
    OutputEntry,
    POPSCreateAccount,
    POPSLoginView,
    POPSStatusWidget,
    PagedContainerData,
    ParametricSelect,
    ParticipantInfo,
    ParticleUserData,
    PatternItem,
    PauseMenu,
    PdxCoreSetting,
    PdxGuiFoldOut,
    PdxGuiGfxVideoControl,
    PdxGuiTableRow,
    PdxGuiTreeTable,
    PdxGuiWidget,
    PdxSetting,
    PdxSettingsWindow,
    PdxSettingsWindowCategory,
    PdxValueSetting,
    Perk,
    PerkGuiTree,
    PerkLineConnection,
    PerkLineItem,
    PlaceRallyPoint,
    PlannedActivityPhase,
    Playable,
    PlayerJoinRequest,
    PlayerMessageItem,
    PlayerValueItem,
    PlotLine,
    Portrait3dView,
    PortraitDataContext,
    PortraitEditorAnimationItem,
    PortraitEditorWindow,
    PortraitTooltip,
    PotentialAgentWindow,
    PotentialCouncillorWindow,
    PotentialElector,
    PotentialFactionMemberWindow,
    PotentialTaskLocationWindow,
    PreviewMaskTexture,
    ProgressInterface,
    Province,
    ProvinceIcon,
    ProvinceMovementAttritionIcon,
    Raid,
    RaidWindow,
    RallyPoint,
    RallyPointItem,
    RallyPointMapIcon,
    RallyPointWindow,
    RankedRoyalCourtItem,
    ReasonItem,
    ReforgeArtifactWindow,
    Regiment,
    RegimentCombatStats,
    RegimentReorgEntry,
    RegimentTerrainModifier,
    RegimentWinterModifier,
    Religion,
    ReligionFamily,
    ReligionWindow,
    RemoveFriendConfirmWindow,
    RenamePopup,
    RepackWindow,
    ReplacePillarWindow,
    ResignConfirmationWindow,
    RevokeTitlesInteractionWindow,
    RoyalCourtScreenshotWindow,
    RoyalCourtWindow,
    RulerDesignerLoadWindow,
    RulerDesignerPortraitModifier,
    RulerDesignerSaveWindow,
    RulerDesignerSkill,
    RulerDesignerWindow,
    SAIActivityInfo,
    SAIActivityLocationInfo,
    SAIActivityOptionInfo,
    SAIActivityPhaseInfo,
    SAICBTypeInfo,
    SAISchemeTypeInfo,
    SAIStrategyInfo,
    SAIValueInfo,
    Savable,
    SavableGroup,
    SaveDialog,
    SaveGame,
    SaveGameAnalysisView,
    SaveGameAnalyzer,
    SaveGameBlockData,
    SaveGameConfigView,
    SaveGameItem,
    SaveGameListView,
    SaveGameWindow,
    SaveRulerItem,
    SaveRulerSkillGui,
    SaveRulerTraitGui,
    Scheme,
    SchemeAgentItem,
    SchemeItem,
    SchemeType,
    Scope,
    ScopeDebugData,
    ScopeDebugInspectorPlugin,
    ScopeObjectEditor,
    ScopeObjectProvider,
    ScopeObjectType,
    ScriptRunnerInspector,
    ScriptRunnerResult,
    ScriptedGui,
    ScriptedRelation,
    SearchListNodeWindow,
    Secret,
    SecretType,
    SelectMenAtArmsOriginView,
    SelectParticleUserDataDialog,
    SelectTool,
    SelectableTaskLocation,
    SelectedRallyPointItem,
    SelectedUnitItem,
    SelectionHistory,
    SelectionLine,
    ServerInformation,
    SettingCategory,
    SettingsPage,
    Siege,
    SiegeWindow,
    SkillItem,
    SkillSchemeGroup,
    SkinEditor,
    SmartBrushHeightRange,
    SmartBrushPattern,
    SmartBrushPresetManager,
    SmartMaterialPaintingMode,
    Social,
    SocialNotificationWindow,
    SocialUI,
    SocialWidget,
    SplineAdjustmentTool,
    SplineAdjustmentToolMode,
    SplineRiverInteractionMode,
    SplineStripTool,
    SplineStripToolMode,
    SplineToolsMapContentPanel,
    SplineTypeCreateSelectionDropdown,
    SplineTypeItem,
    SplineTypeSwitchSelectionDropdown,
    SplineVisibilityDropdown,
    StaticModifier,
    Story,
    StoryInfo,
    StraitMapIcon,
    Struggle,
    StruggleInvolvementWindow,
    StrugglePhase,
    StruggleType,
    StruggleWindow,
    SuccessionElectionWindow,
    SuccessionElectionWindowCandidate,
    SuccessionElectionWindowElector,
    SuccessionElectionWindowElectorVote,
    SuccessionElector,
    SuccessionEventWindow,
    SuccessionEventWindowLostTitlesItem,
    SuccessionLawChangeWindow,
    SuggestionItem,
    SuggestionType,
    Terrain,
    TerrainToolButton,
    TextureEntry,
    TextureImporter,
    TextureList,
    TextureListDirectory,
    TextureListTexture,
    TextureNodeWindow,
    TextureViewer,
    TimelineKeyframe,
    TimelineWidget,
    Title,
    TitleAddLawWindow,
    TitleCapitalMapIcon,
    TitleCustomizationWindow,
    TitleHistory,
    TitleHistoryWindow,
    TitleItem,
    TitleSuccessionItem,
    TitleViewWindow,
    ToastMessageHandler,
    TokenParameter,
    ToolDialog,
    ToolDialogButton,
    ToolMessageDialog,
    ToolProperty,
    ToolPropertyBool,
    ToolPropertyCColor,
    ToolPropertyCString,
    ToolPropertyColor,
    ToolPropertyCurve,
    ToolPropertyFloat,
    ToolPropertyInt,
    ToolPropertyInt16,
    ToolPropertyInt8,
    ToolPropertyList,
    ToolPropertySearchList,
    ToolPropertyString,
    ToolPropertyUint,
    ToolPropertyUint16,
    ToolPropertyUint8,
    ToolPropertyUndoableSearchList,
    ToolPropertyVec1fPercent,
    ToolPropertyVec2f,
    ToolPropertyVec2fPercent,
    ToolPropertyVec2i,
    ToolPropertyVec3f,
    ToolPropertyVec3i,
    ToolPropertyVec4i,
    ToolsPropertyRangedValueFloat,
    ToolsPropertyRangedValueInt,
    ToolsSearch,
    ToolsSearchResult,
    ToolsUndoableValueBundleBool,
    ToolsUndoableValueBundleCColor,
    ToolsUndoableValueBundleCString,
    ToolsUndoableValueBundleColor,
    ToolsUndoableValueBundleFloat,
    ToolsUndoableValueBundleInt,
    ToolsUndoableValueBundleString,
    ToolsUndoableValueBundleUint,
    TooltipInfo,
    TopScope,
    TraditionGrouping,
    Trait,
    TraitArrays,
    TraitGroup,
    TraitLevelTrack,
    TraitLevelTrackEntry,
    TraitSlot,
    TraitSlotArray,
    TransferVassalWindow,
    TravelDangerMapIconData,
    TravelDangerType,
    TravelOption,
    TravelOptionSelectionWindow,
    TravelPlan,
    TravelPlanCompanion,
    TravelPlanData,
    TravelPlanDestination,
    TravelPlanDraft,
    TravelPlanPath,
    TravelPlanWaypoint,
    TravelPlanner,
    TravelPointOfInterestType,
    TravelPointsOfInterestIconData,
    TravelProvinceMapIcon,
    TravelRouteEditWindow,
    TroopItem,
    Tutorial,
    TutorialWindow,
    Tweakable,
    TweakableCategory,
    TweakableUiEntry,
    Tweaker,
    Type,
    UndoStack,
    UniqueBuildingIcon,
    UnitItem,
    UnitMapIcon,
    UnitMapIconHandler,
    UserDataNode,
    ValueBreakdown,
    VariableEntry,
    VariableInfo,
    VariableInspectorEntry,
    VariableInspectorPlugin,
    VariableInspectorVariable,
    VariableList,
    VariableListEntry,
    VariableListInspectorPlugin,
    VariableListStore,
    VariableStore,
    VariableSystem,
    VassalContract,
    VassalContractType,
    VassalConversionWindow,
    VassalStance,
    VassalStanceFilter,
    Vec4f,
    ViewerEntity,
    ViewerEntityLodInfo,
    ViewerEntityState,
    War,
    WarAllyItem,
    WarDeclaredOverviewWindow,
    WarInfo,
    WarItem,
    WarOverviewWindow,
    WarParticipantItem,
    WarResultsWindow,
    WatchWindow,
    bool,
    double,
    float,
    int16,
    int32,
    int64,
    int8,
    uint16,
    uint32,
    uint64,
    uint8,
    void,
}
