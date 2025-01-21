import test from 'ava'

import { escape_html } from '../index'

test('sync function from native code', (t) => {
  t.is(escape_html('<div>1</div>'), '&lt;div&gt;1&lt;&#x2f;div&gt;')
})
