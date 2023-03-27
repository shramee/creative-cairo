(() => {
  const canvas = document.getElementById('canvas');
  var ctx = canvas.getContext('2d');
  const editor = document.querySelector('#cairo-editor');
  const starterCode =
    "// You can draw an arc or circle,\n// 'use' (equivalent to import) them from draw\nuse draw::circle;\nuse draw::arc;\n\n// Circle is also an arc\nuse draw::Arc;\n\n// Set return tuple with all shapes you are returning\nfn main() -> (Arc, Arc) {\n  (\n    circle( 384, 256, 100 ),\n    circle( 128, 256, 100 ),\n  )\n}";
  editor.innerHTML = starterCode;
  CodeMirror.fromTextArea(document.getElementById('cairo-editor'), {
    // extensions: [basicSetup, rust()],
    lineNumbers: true,
    border: true,
    theme: 'material',
    mode: 'application/json',
    gutters: ['CodeMirror-lint-markers'],
    styleActiveLine: true,
    lint: true,
  });

  editor.dispatchEvent(new Event('change', { bubbles: true }));
  window.submitCairo = async () => {
    let cairo = document.getElementById('cairo-editor').value;
    let cairoResp = await fetch('./cairo', {
      method: 'POST',
      headers: {},
      body: cairo,
    });
    cairoResp = await cairoResp.text();

    document.getElementById('response').innerHTML = cairoResp
      .replace('[', '<span>')
      .replace(']', '</span>')
      .split(', ')
      .join(',</span><span>');
    processCairoResp(JSON.parse(cairoResp));
  };

  /*
   * Converts felt to string
   */
  const feltToString = (felt) =>
    felt
      .toString(16) // To hex
      .match(/.{2}/g) // Split into 2 chars
      .map((c) => String.fromCharCode(parseInt(c, 16))) // Get char from code
      .join(''); // Join to a string

  const actionHandlers = {
    arc(x, y, r, s, e) {
      s = (2 * Math.PI * s) / 10000;
      e = (2 * Math.PI * e) / 10000;
      ctx.beginPath();
      ctx.arc(x, y, r, s, e);
      ctx.stroke();
    },
  };

  function handleAction(action, args) {
    action = feltToString(+action);
    console.log(action, args);
    if (typeof actionHandlers[action] === 'function') {
      actionHandlers[action](...args);
    } else if (typeof ctx[action] === 'function') {
      ctx[action](...args);
    } else if (typeof ctx[action] !== 'undefined' && args.length === 1) {
      ctx[action] = args[0];
    }
  }

  function processCairoResp(cairoResp) {
    let dt = cairoResp;
    if (typeof cairoResp === 'string') {
      dt = cairoResp.replace(/,$/, '').split(',');
    }

    ctx.fillStyle = '#fcfaf6';
    ctx.fillRect(0, 0, 9999, 9999);
    ctx.fillStyle = '#fe4a49';
    ctx.strokeStyle = '#fe4a49';

    callbacks = {};

    for (let i = 0; i < dt.length; i++) {
      const action = dt[i];
      const n_args = +dt[i + 1];
      console.log(action, n_args);
      const args = dt.slice(i + 2, i + 2 + n_args);
      handleAction(action, args);
      i += n_args + 1;
    }
  }
})();
