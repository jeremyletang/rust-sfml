#pragma once

typedef enum {
    sfLineAlignmentDefault,
    sfLineAlignmentLeft,
    sfLineAlignmentCenter,
    sfLineAlignmentRight,
} sfLineAlignment;

typedef enum {
    sfTextOrientationDefault,
    sfTextOrientationTopToBottom,
    sfTextOrientationBottomToTop,
} sfTextOrientation;

typedef enum {
    sfClusterGroupingGrapheme,
    sfClusterGroupingCharacter,
    sfClusterGroupingNone,
} sfClusterGrouping;
