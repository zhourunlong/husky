[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`malamute::Class`, `Enum`),
            ),
        ),
        Ok(
            DecTerm(`independent Type -> Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`malamute::OneVsAll`, `Enum`),
            ),
        ),
        Ok(
            DecTerm(`(independent v0: Type) -> independent v0 -> Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`malamute::OneVsAllResult`, `Enum`),
            ),
        ),
        Ok(
            DecTerm(`(independent v0: Type) -> independent v0 -> Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
            ),
        ),
        Ok(
            DecTerm(`(independent v1: Type) -> (independent v0: v1) -> fn((core::num::f32, core::num::i32) -> malamute::OneVsAllResult v1 v0`),
        ),
    ),
]