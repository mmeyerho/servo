import util::color::Color;

enum DisplayType {
    DisBlock,
    DisInline,
    DisNone
}

enum Unit {
    Percent(float),
    In(float),
    Mm(float),
    Cm(float),
    Em(float),
    Ex(float), //TODO: find out what ex is 
    Pt(uint),
    Pc(uint),
    Px(uint)
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
