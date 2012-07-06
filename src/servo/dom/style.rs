import util::color::Color;

enum DisplayType {
    DisBlock,
    DisInline,
    DisNone
}

enum Unit {
    Auto,
    Percent(float),
    In(float),
    Mm(float),
    Cm(float),
    Em(float),
    Ex(float),
    Pt(float),
    Pc(float),
    Px(float)
}

enum StyleDeclaration {
    BackgroundColor(Color),
    Display(DisplayType),
    FontSize(Unit),
    Height(Unit),
    TextColor(Color),
    Width(Unit)
}

enum Attr {
    Exists(str),
    Exact(str, str),
    Includes(str, str),
    StartsWith(str, str)
}
    
enum Selector {
    Element(str, [Attr]),
    Child(~Selector, ~Selector),
    Descendant(~Selector, ~Selector),
    Sibling(~Selector, ~Selector)
}

type Rule = ([~Selector], [StyleDeclaration]);

type Stylesheet = [~Rule];
