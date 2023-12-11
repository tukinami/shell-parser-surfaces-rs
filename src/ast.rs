//! AST for `SERIKO`.

use shell_parser_common_rs::charset::Charset;

pub type SurfaceSizeType = u32;
pub type CoordinateType = i64;
pub type SurfaceIdType = u32;
pub type SurfaceIdPointerType = i64;
pub type AnimationIdType = u32;
pub type ElementIdType = AnimationIdType;
pub type CollisionIdType = AnimationIdType;
pub type AnimationPatternIdType = AnimationIdType;
pub type CursorMouseIdType = AnimationIdType;

/// Root of `SERIKO`.
#[derive(Debug, Clone, PartialEq)]
pub struct Seriko {
    header_comments: Vec<CommentLine>,
    charset: Charset,
    braces: Vec<BraceContainer>,
    footer_comments: Vec<CommentLine>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommentLine {
    body: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineContainer<T> {
    Comment(CommentLine),
    Body(T),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BraceContainer {
    header_comments: Vec<CommentLine>,
    body: Brace,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Brace {
    Descript(Descript),
    Surface(Surface),
    SurfaceAppend(SurfaceAppend),
    SurfaceAlias(SurfaceAlias),
    Cursor(SerikoCursor),
    Tooltip(Tooltip),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Descript {
    lines: Vec<LineContainer<DescriptInner>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DescriptInner {
    Version(u16),
    MaxWidth(SurfaceSizeType),
    CollistionSort(SortOrder),
    AnimationSort(SortOrder),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortOrder {
    Ascend,
    Descend,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Surface {
    ids: Vec<SurfaceId>,
    lines: Vec<LineContainer<SurfaceInner>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceAppend {
    ids: Vec<SurfaceId>,
    lines: Vec<LineContainer<SurfaceInner>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceId {
    Unit(SurfaceIdType),
    Range(SurfaceIdType, SurfaceIdType),
    Not(Box<SurfaceId>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceInner {
    Element(SurfaceElement),
    AnimationInterval(SurfaceAnimationInterval),
    AnimationPattern(SurfaceAnimationPattern),
    AnimationOption(SurfaceAnimationOption),
    AnimationCollision(SurfaceAnimationCollision),
    AnimationCollisionEx(SurfaceAnimationCollisionEx),
    Collision(SurfaceCollision),
    CollisionEx(SurfaceCollisionEx),
    SakuraBalloonOffsetX(CoordinateType),
    SakuraBalloonOffsetY(CoordinateType),
    KeroBalloonOffsetX(CoordinateType),
    KeroBalloonOffsetY(CoordinateType),
    BalloonOffsetX(CoordinateType),
    BalloonOffsetY(CoordinateType),
    PointCenterX(CoordinateType),
    PointCenterY(CoordinateType),
    PointKinokoCenterX(CoordinateType),
    PointKinokoCenterY(CoordinateType),
    PointBaseposX(CoordinateType),
    PointBaseposY(CoordinateType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceElement {
    id: ElementIdType,
    method: DrawMethod,
    filename: String,
    x: CoordinateType,
    y: CoordinateType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawMethod {
    Base,
    Overlay,
    Overlayfast,
    Overlaymultiply,
    Replace,
    Interpolate,
    Asis,
    Move,
    Bind,
    Add,
    Reduce,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawMethodOnAnimation {
    Insert(AnimationIdType),
    Start(AnimationIdType),
    Stop(AnimationIdType),
    Alternativestart(Vec<AnimationIdType>),
    Alternativestop(Vec<AnimationIdType>),
    Parallelstart(Vec<AnimationIdType>),
    Parallelstop(Vec<AnimationIdType>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceAnimationInterval {
    id: AnimationIdType,
    intervals: Vec<AnimationInterval>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationInterval {
    Sometimes,
    Rarely,
    Random(u32),
    Periodic(u32),
    Always,
    Runonce,
    Never,
    YenE,
    Talk(u32),
    Bind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceAnimationPattern {
    id: AnimationIdType,
    pattern_id: AnimationPatternIdType,
    method: AnimationPatternDrawMethod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationPatternDrawMethod {
    Normal(DrawMethod, AnimationPatternProperty),
    Animation(DrawMethodOnAnimation),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationPatternProperty {
    surface_id: SurfaceIdPointerType,
    weight: u32,
    x: CoordinateType,
    y: CoordinateType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceAnimationOption {
    id: AnimationIdType,
    options: Vec<AnimationOptionKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationOptionKind {
    Exclusive(Option<Vec<AnimationIdType>>),
    Background,
    SharedIndex,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceAnimationCollision {
    id: AnimationIdType,
    collision: SurfaceCollision,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceAnimationCollisionEx {
    id: AnimationIdType,
    collision: SurfaceCollisionEx,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceCollision {
    id: CollisionIdType,
    start_x: CoordinateType,
    start_y: CoordinateType,
    end_x: CoordinateType,
    end_y: CoordinateType,
    target_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceCollisionEx {
    id: CollisionIdType,
    target_id: String,
    kind: CollisionExKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CollisionExKind {
    Rect(
        CoordinateType,
        CoordinateType,
        CoordinateType,
        CoordinateType,
    ),
    Ellipse(
        CoordinateType,
        CoordinateType,
        CoordinateType,
        CoordinateType,
    ),
    Circle(CoordinateType, CoordinateType, CoordinateType),
    Polygon(Vec<CoordinateType>),
    Region(String, u8, u8, u8, Option<bool>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceAlias {
    id: SurfaceTargetCharacterId,
    lines: Vec<LineContainer<SurfaceAliasInner>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceTargetCharacterId {
    Sakura,
    Kero,
    Char(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceAliasInner {
    target: String,
    surfaces: Vec<SurfaceIdType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SerikoCursor {
    id: SurfaceTargetCharacterId,
    lines: Vec<LineContainer<SerikoCursorGesture>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SerikoCursorGesture {
    kind: GestureKind,
    id: CursorMouseIdType,
    target_collistion: String,
    filename: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GestureKind {
    MouseUp,
    MouseDown,
    MouseRightDown,
    MouseWheel,
    MouseHover,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tooltip {
    id: SurfaceTargetCharacterId,
    lines: Vec<LineContainer<TooltipInner>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TooltipInner {
    collision: String,
    description: String,
}

impl Seriko {
    pub fn new(
        header_comments: Vec<CommentLine>,
        charset: Charset,
        braces: Vec<BraceContainer>,
        footer_comments: Vec<CommentLine>,
    ) -> Seriko {
        Seriko {
            header_comments,
            charset,
            braces,
            footer_comments,
        }
    }

    pub fn header_comments(&self) -> &Vec<CommentLine> {
        &self.header_comments
    }
    pub fn charset(&self) -> &Charset {
        &self.charset
    }
    pub fn braces(&self) -> &Vec<BraceContainer> {
        &self.braces
    }
    pub fn footer_comments(&self) -> &Vec<CommentLine> {
        &self.footer_comments
    }
}

impl CommentLine {
    pub fn new(body: String) -> CommentLine {
        CommentLine { body }
    }

    pub fn body(&self) -> &String {
        &self.body
    }
}

impl BraceContainer {
    pub fn new(header_comments: Vec<CommentLine>, body: Brace) -> BraceContainer {
        BraceContainer {
            header_comments,
            body,
        }
    }

    pub fn header_comments(&self) -> &Vec<CommentLine> {
        &self.header_comments
    }
    pub fn body(&self) -> &Brace {
        &self.body
    }
}

impl Descript {
    pub fn new(lines: Vec<LineContainer<DescriptInner>>) -> Descript {
        Descript { lines }
    }

    pub fn lines(&self) -> &Vec<LineContainer<DescriptInner>> {
        &self.lines
    }
}

impl Surface {
    pub fn new(ids: Vec<SurfaceId>, lines: Vec<LineContainer<SurfaceInner>>) -> Surface {
        Surface { ids, lines }
    }

    pub fn ids(&self) -> &Vec<SurfaceId> {
        &self.ids
    }
    pub fn lines(&self) -> &Vec<LineContainer<SurfaceInner>> {
        &self.lines
    }
}

impl SurfaceAppend {
    pub fn new(ids: Vec<SurfaceId>, lines: Vec<LineContainer<SurfaceInner>>) -> SurfaceAppend {
        SurfaceAppend { ids, lines }
    }

    pub fn ids(&self) -> &Vec<SurfaceId> {
        &self.ids
    }
    pub fn lines(&self) -> &Vec<LineContainer<SurfaceInner>> {
        &self.lines
    }
}

impl SurfaceElement {
    pub fn new(
        id: ElementIdType,
        method: DrawMethod,
        filename: String,
        x: CoordinateType,
        y: CoordinateType,
    ) -> SurfaceElement {
        SurfaceElement {
            id,
            method,
            filename,
            x,
            y,
        }
    }

    pub fn id(&self) -> &ElementIdType {
        &self.id
    }
    pub fn method(&self) -> &DrawMethod {
        &self.method
    }
    pub fn filename(&self) -> &String {
        &self.filename
    }
    pub fn x(&self) -> &CoordinateType {
        &self.x
    }
    pub fn y(&self) -> &CoordinateType {
        &self.y
    }
}

impl SurfaceAnimationInterval {
    pub fn new(id: AnimationIdType, intervals: Vec<AnimationInterval>) -> SurfaceAnimationInterval {
        SurfaceAnimationInterval { id, intervals }
    }

    pub fn id(&self) -> &AnimationIdType {
        &self.id
    }
    pub fn intervals(&self) -> &Vec<AnimationInterval> {
        &self.intervals
    }
}

impl SurfaceAnimationPattern {
    pub fn new(
        id: AnimationIdType,
        pattern_id: AnimationPatternIdType,
        method: AnimationPatternDrawMethod,
    ) -> SurfaceAnimationPattern {
        SurfaceAnimationPattern {
            id,
            pattern_id,
            method,
        }
    }

    pub fn id(&self) -> &AnimationIdType {
        &self.id
    }
    pub fn pattern_id(&self) -> &AnimationPatternIdType {
        &self.pattern_id
    }
    pub fn method(&self) -> &AnimationPatternDrawMethod {
        &self.method
    }
}

impl AnimationPatternProperty {
    pub fn new(
        surface_id: SurfaceIdPointerType,
        weight: u32,
        x: CoordinateType,
        y: CoordinateType,
    ) -> AnimationPatternProperty {
        AnimationPatternProperty {
            surface_id,
            weight,
            x,
            y,
        }
    }

    pub fn surface_id(&self) -> &SurfaceIdPointerType {
        &self.surface_id
    }
    pub fn weight(&self) -> &u32 {
        &self.weight
    }
    pub fn x(&self) -> &CoordinateType {
        &self.x
    }
    pub fn y(&self) -> &CoordinateType {
        &self.y
    }
}

impl SurfaceAnimationOption {
    pub fn new(id: AnimationIdType, options: Vec<AnimationOptionKind>) -> SurfaceAnimationOption {
        SurfaceAnimationOption { id, options }
    }

    pub fn id(&self) -> &AnimationIdType {
        &self.id
    }
    pub fn options(&self) -> &Vec<AnimationOptionKind> {
        &self.options
    }
}

impl SurfaceAnimationCollision {
    pub fn new(id: AnimationIdType, collision: SurfaceCollision) -> SurfaceAnimationCollision {
        SurfaceAnimationCollision { id, collision }
    }

    pub fn id(&self) -> &AnimationIdType {
        &self.id
    }
    pub fn collision(&self) -> &SurfaceCollision {
        &self.collision
    }
}

impl SurfaceAnimationCollisionEx {
    pub fn new(id: AnimationIdType, collision: SurfaceCollisionEx) -> SurfaceAnimationCollisionEx {
        SurfaceAnimationCollisionEx { id, collision }
    }

    pub fn id(&self) -> &AnimationIdType {
        &self.id
    }
    pub fn collision(&self) -> &SurfaceCollisionEx {
        &self.collision
    }
}

impl SurfaceCollision {
    pub fn new(
        id: CollisionIdType,
        start_x: CoordinateType,
        start_y: CoordinateType,
        end_x: CoordinateType,
        end_y: CoordinateType,
        target_id: String,
    ) -> SurfaceCollision {
        SurfaceCollision {
            id,
            start_x,
            start_y,
            end_x,
            end_y,
            target_id,
        }
    }

    pub fn id(&self) -> &CollisionIdType {
        &self.id
    }
    pub fn start_x(&self) -> &CoordinateType {
        &self.start_x
    }
    pub fn start_y(&self) -> &CoordinateType {
        &self.start_y
    }
    pub fn end_x(&self) -> &CoordinateType {
        &self.end_x
    }
    pub fn end_y(&self) -> &CoordinateType {
        &self.end_y
    }
    pub fn target_id(&self) -> &String {
        &self.target_id
    }
}

impl SurfaceCollisionEx {
    pub fn new(
        id: CollisionIdType,
        target_id: String,
        kind: CollisionExKind,
    ) -> SurfaceCollisionEx {
        SurfaceCollisionEx {
            id,
            target_id,
            kind,
        }
    }

    pub fn id(&self) -> &CollisionIdType {
        &self.id
    }
    pub fn target_id(&self) -> &String {
        &self.target_id
    }
    pub fn kind(&self) -> &CollisionExKind {
        &self.kind
    }
}

impl SurfaceAlias {
    pub fn new(
        id: SurfaceTargetCharacterId,
        lines: Vec<LineContainer<SurfaceAliasInner>>,
    ) -> SurfaceAlias {
        SurfaceAlias { id, lines }
    }

    pub fn id(&self) -> &SurfaceTargetCharacterId {
        &self.id
    }
    pub fn lines(&self) -> &Vec<LineContainer<SurfaceAliasInner>> {
        &self.lines
    }
}

impl SurfaceAliasInner {
    pub fn new(target: String, surfaces: Vec<SurfaceIdType>) -> SurfaceAliasInner {
        SurfaceAliasInner { target, surfaces }
    }

    pub fn target(&self) -> &String {
        &self.target
    }
    pub fn surfaces(&self) -> &Vec<SurfaceIdType> {
        &self.surfaces
    }
}

impl SerikoCursor {
    pub fn new(
        id: SurfaceTargetCharacterId,
        lines: Vec<LineContainer<SerikoCursorGesture>>,
    ) -> SerikoCursor {
        SerikoCursor { id, lines }
    }

    pub fn id(&self) -> &SurfaceTargetCharacterId {
        &self.id
    }
    pub fn lines(&self) -> &Vec<LineContainer<SerikoCursorGesture>> {
        &self.lines
    }
}

impl SerikoCursorGesture {
    pub fn new(
        kind: GestureKind,
        id: CursorMouseIdType,
        target_collistion: String,
        filename: String,
    ) -> SerikoCursorGesture {
        SerikoCursorGesture {
            kind,
            id,
            target_collistion,
            filename,
        }
    }

    pub fn kind(&self) -> &GestureKind {
        &self.kind
    }
    pub fn id(&self) -> &CursorMouseIdType {
        &self.id
    }
    pub fn target_collistion(&self) -> &String {
        &self.target_collistion
    }
    pub fn filename(&self) -> &String {
        &self.filename
    }
}

impl Tooltip {
    pub fn new(id: SurfaceTargetCharacterId, lines: Vec<LineContainer<TooltipInner>>) -> Tooltip {
        Tooltip { id, lines }
    }

    pub fn id(&self) -> &SurfaceTargetCharacterId {
        &self.id
    }
    pub fn lines(&self) -> &Vec<LineContainer<TooltipInner>> {
        &self.lines
    }
}

impl TooltipInner {
    pub fn new(collision: String, description: String) -> TooltipInner {
        TooltipInner {
            collision,
            description,
        }
    }

    pub fn collision(&self) -> &String {
        &self.collision
    }
    pub fn description(&self) -> &String {
        &self.description
    }
}
