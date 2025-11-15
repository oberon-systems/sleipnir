use smallvec::SmallVec;

pub struct GraphiteMetric<'a> {
    pub name: &'a str,
    pub tags: SmallVec<[(&'a str, &'a str); 16]>,
    pub value: f64,
    pub timestamp: i64,
}

impl<'a> GraphiteMetric<'a> {
    #[inline(always)]
    pub fn parse(line: &'a str) -> Result<Self, String> {
        let bytes = line.as_bytes();
        let len = bytes.len();

        // try to search first two spaces
        let (mut sp1, mut sp2) = (usize::MAX, usize::MAX);

        for i in 0..len {
            if bytes[i] == b' ' {
                if sp1 == usize::MAX {
                    sp1 = i;
                } else {
                    sp2 = i;
                    break;
                }
            }
        }
        if sp1 == usize::MAX || sp2 == usize::MAX {
            return Err(format!("invalid graphite line: {:?}", line));
        }

        // collect metric (path), value and timestamp
        let metric = &line[..sp1];
        let value_str = &line[sp1 + 1 .. sp2];
        let ts_str = &line[sp2 + 1 ..];

        // convert value and timestamp
        let value: f64 = value_str.parse().map_err(|_| "bad value")?;
        let timestamp: i64 = ts_str.parse().map_err(|_| "bad ts")?;

        // parse metric (path)
        let mb = metric.as_bytes();
        let mlen = mb.len();
        let mut i = 0;

        // collect name
        while i < mlen && mb[i] != b';' {
            i += 1;
        }
        let name = &metric[..i];

        // ... collect tags
        let mut tags = SmallVec::<[(&str, &str); 16]>::new();

        while i < mlen {
            i += 1; // skip ';'
            if i >= mlen { break; }

            let key_start = i;
            while i < mlen && mb[i] != b'=' { i += 1; }
            if i >= mlen { break; }
            let key_end = i;

            i += 1; // skip '='
            if i >= mlen { break; }

            let val_start = i;
            while i < mlen && mb[i] != b';' { i += 1; }
            let val_end = i;

            tags.push((
                &metric[key_start..key_end],
                &metric[val_start..val_end],
            ));
        }

        Ok(Self { name, tags, value, timestamp })
    }
}
