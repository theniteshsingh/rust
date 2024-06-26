// Test that `-Cinstrument-coverage` creates expected __llvm_profile_filename symbol in LLVM IR.

//@ needs-profiler-support
//@ revisions: default y yes on true_ all
//@ [default] compile-flags: -Cinstrument-coverage
//@ [y] compile-flags: -Cinstrument-coverage=y
//@ [yes] compile-flags: -Cinstrument-coverage=yes
//@ [on] compile-flags: -Cinstrument-coverage=on
//@ [true_] compile-flags: -Cinstrument-coverage=true
//@ [all] compile-flags: -Cinstrument-coverage=all

// CHECK: @__llvm_profile_filename = {{.*}}"default_%m_%p.profraw\00"{{.*}}
// CHECK: @__llvm_coverage_mapping

#![crate_type = "lib"]

#[inline(never)]
fn some_function() {}

pub fn some_other_function() {
    some_function();
}
