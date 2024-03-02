import init, { get_all_date_recurrences_between } from '../../../pkg/rrule.js';

async function executeRRulePerformanceTest(ruleSet, after, before, limit) {
  var rruleWork = () => {
    const rule = new rrule.RRule.fromString(ruleSet);
    const results = rule.between(after, before);
  }
  return executeWork(rruleWork, "rrule");
}
async function executeRustRRulePerformanceTest(ruleSet, after, before, limit) {
  await init();
  var rustWork = () => {
    const data = get_all_date_recurrences_between(ruleSet, limit, after, before);
  }
  return executeWork(rustWork, "rust-rrule");
}
function executeWork(work, framework) {
  var performance = window.performance;
  var t0 = performance.now();
  var i = 0;
  const times = 100;
  for (i = 0; i < times; ++i) {
    work();
  }
  var t1 = performance.now();
  const result = "Call to " + framework  + " took " + (t1 - t0)/times + " milliseconds.";
  return result;
}

async function executePerformanceTests() {
    const ruleSet = document.getElementById("ruleSet").value.replace('\\n', '\n');
    const afterDateString = document.getElementById("after").value;
    const beforeDateString = document.getElementById("before").value;
    const limit = document.getElementById("limit").value;
    let after = new Date(afterDateString);
    let before = new Date(beforeDateString)
    const rustRRuleResultDiv = document.querySelector("#rustRRuleResult");
    rustRRuleResultDiv.innerHTML = "Executing ...";
    await executeRustRRulePerformanceTest(ruleSet, after, before, limit).then((value) => { 
      rustRRuleResultDiv.innerHTML = value;
    });
    const rruleResultDiv = document.querySelector("#rruleResult");
    rruleResultDiv.innerHTML = "Executing ...";
    await executeRRulePerformanceTest(ruleSet, after, before, limit).then((value) => { 
      rruleResultDiv.innerHTML = value;
    });
}

document.addEventListener("DOMContentLoaded", () => {
    const performanceButton = document.querySelector("#performanceButton");
  
    performanceButton.addEventListener("click", () => {
        executePerformanceTests();
    });
});
