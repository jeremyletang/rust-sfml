#pragma once
enum sfCursorType {
    Arrow,                  //!< Arrow cursor (default)
    ArrowWait,              //!< Busy arrow cursor
    Wait,                   //!< Busy cursor
    Text,                   //!< I-beam, cursor when hovering over a field allowing text entry
    Hand,                   //!< Pointing hand cursor
    SizeHorizontal,         //!< Horizontal double arrow cursor
    SizeVertical,           //!< Vertical double arrow cursor
    SizeTopLeftBottomRight, //!< Double arrow cursor going from top-left to bottom-right
    SizeBottomLeftTopRight, //!< Double arrow cursor going from bottom-left to top-right
    SizeLeft,               //!< Left arrow cursor on Linux, same as SizeHorizontal on other platforms
    SizeRight,              //!< Right arrow cursor on Linux, same as SizeHorizontal on other platforms
    SizeTop,                //!< Up arrow cursor on Linux, same as SizeVertical on other platforms
    SizeBottom,             //!< Down arrow cursor on Linux, same as SizeVertical on other platforms
    SizeTopLeft,            //!< Top-left arrow cursor on Linux, same as SizeTopLeftBottomRight on other platforms
    SizeBottomRight,        //!< Bottom-right arrow cursor on Linux, same as SizeTopLeftBottomRight on other platforms
    SizeBottomLeft,         //!< Bottom-left arrow cursor on Linux, same as SizeBottomLeftTopRight on other platforms
    SizeTopRight,           //!< Top-right arrow cursor on Linux, same as SizeBottomLeftTopRight on other platforms
    SizeAll,                //!< Combination of SizeHorizontal and SizeVertical
    Cross,                  //!< Crosshair cursor
    Help,                   //!< Help cursor
    NotAllowed              //!< Action not allowed cursor
};
