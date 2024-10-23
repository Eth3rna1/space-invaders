use crate::engine::Engine;

/// Returns a pool of coordinates
fn draw_line(engine : &mut Engine, _start : (usize, usize), _end : (usize, usize)) -> Result<Vec<(usize, usize)>, String> {
    // I am not understanding the concept
    // I need to draw paper examples
    //    Todo: find the next coordinate point between a start and end point
    //
    //     Matrix: (+ start)  (- end)
    //     |---------------------------|
    //     |                           |
    //     |    +?                     |
    //     |      ?                    |
    //     |       ?                   |
    //     |        ?????              |
    //     |             ?             |
    //     |              ? ??         |
    //     |                  ? -      |
    //     |                           |
    //     |                           |
    //     |---------------------------|
    //
    //
    //     Answer:
    //         define the left coordinate and right coordinate from start and end
    //         and get the slope
    //
    //     Formula:
    //             rise      y2 - y1
    //            ------  = --------- = m (slope)
    //              run      x2 - x2
    todo!()
}
