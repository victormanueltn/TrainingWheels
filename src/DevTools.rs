pub struct Snippets {
    pub initial: String,
}

impl Snippets {
    pub fn new() -> Snippets {
        Snippets {
            initial: r#"#include "base/base.h"
#include "vdm/vdm.h"

int main()
{
    return 0;
}
"#
            .to_string(),
        }
    }
}
