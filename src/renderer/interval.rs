use crate::utility::constants::INFINITY;

pub struct Interval
{
    _min: f32,
    _max: f32
}

impl Interval
{
    pub fn new(min: f32, max: f32) -> Self
    {
        Interval
        {
            _min: min,
            _max: max
        }
    }

    pub fn contains(&self, x: f32) -> bool
    {
        self._min <= x && self._max >= x
    }

    pub fn surrounds(&self, x: f32) -> bool
    {
        self._min < x && self._max > x
    }

    pub fn max(&self) -> f32
    {
        self._max
    }

    pub fn min(&self) -> f32
    {
        self._min
    }

    pub fn clamp(&self, x: f32) -> f32
    {
        if x < self._min
        {
            self._min
        }
        else if x > self._max
        {
            self._max
        }
        else
        {
            x
        }
    }

    pub fn empty() -> Self
    {
        Interval
        {
            _min: INFINITY,
            _max: -INFINITY
        }
    }

    pub fn universe() -> Self
    {
        Interval
        {
            _min: -INFINITY,
            _max: INFINITY
        }
    }
}