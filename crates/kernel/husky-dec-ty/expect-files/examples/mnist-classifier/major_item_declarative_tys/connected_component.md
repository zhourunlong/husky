[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
            ),
        ),
        Ok(
            DecTerm(`Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
            ),
        ),
        Ok(
            DecTerm(`Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTerm(`fn((~ mnist_classifier::raw_contour::RawContour) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
            ),
        ),
        Ok(
            DecTerm(`Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTerm(`fn((core::raw_bits::r32, core::raw_bits::r32) -> core::raw_bits::r32`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTerm(`fn((mnist::BinaryImage28) -> [] mnist_classifier::connected_component::ConnectedComponent`),
        ),
    ),
]