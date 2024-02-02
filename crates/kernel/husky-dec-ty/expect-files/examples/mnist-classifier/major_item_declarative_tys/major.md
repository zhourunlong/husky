[
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
            ),
        ),
        Ok(
            DecTerm(`{val_type} [] mnist_classifier::connected_component::ConnectedComponent`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
            ),
        ),
        Ok(
            DecTerm(`{val_type} ~ mnist_classifier::connected_component::ConnectedComponent`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
            ),
        ),
        Ok(
            DecTerm(`{val_type} core::num::f32`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
            ),
        ),
        Ok(
            DecTerm(`{val_type} ~ [] mnist_classifier::raw_contour::RawContour`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
            ),
        ),
        Ok(
            DecTerm(`{val_type} ~ mnist_classifier::raw_contour::RawContour`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
            ),
        ),
        Ok(
            DecTerm(`{val_type} ~ mnist_classifier::line_segment_sketch::LineSegmentSketch`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
            ),
        ),
        Ok(
            DecTerm(`{val_type} ~ [] mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
        ),
    ),
]