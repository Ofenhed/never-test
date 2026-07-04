Using `rustc 1.98.0-nightly (14210df0e 2026-05-31)`, the result of [`test.sh`](test.sh) is:

## Debug mode
```
Feature is ''
cargo run
The type weighs 16 bytes
Container status: true
My container contains container
=== MyContainer strings ===
MyContainer::do_stuff is included in the binary
drop_glue<never_test::MyContainer>
_RNvMCsc7kiMf9cLFg_10never_testNtB2_11MyContainer8do_stuff
_RNvMCsc7kiMf9cLFg_10never_testNtB2_11MyContainer7replace
_RNvMCsc7kiMf9cLFg_10never_testNtB2_11MyContainer8to_inner
MyContainer
Option<never_test::MyContainer>
_RNvMNtCs8b4M6GsfcPr_4core6optionINtB2_6OptionNtCsc7kiMf9cLFg_10never_test11MyContainerE7is_someCslYCV9b1PA5r_10never_test
_RNvXs_Csc7kiMf9cLFg_10never_testNtB4_11MyContainerNtNtNtCs8b4M6GsfcPr_4core3ops5deref5Deref5deref
&mut never_test::MyContainer
_RNvMCsc7kiMf9cLFg_10never_testNtB2_11MyContainer3new
_RINvNtCs8b4M6GsfcPr_4core3ptr9drop_glueNtCsc7kiMf9cLFg_10never_test11MyContainerEBD_
&never_test::MyContainer
is_some<never_test::MyContainer>
&core::option::Option<never_test::MyContainer>
_RNvMNtCs8b4M6GsfcPr_4core6optionINtB2_6OptionNtCsc7kiMf9cLFg_10never_test11MyContainerE7is_someCslYCV9b1PA5r_10never_test
_RNvMCsc7kiMf9cLFg_10never_testNtB2_11MyContainer3new
_RNvMCsc7kiMf9cLFg_10never_testNtB2_11MyContainer8do_stuff
_RINvNtCs8b4M6GsfcPr_4core3ptr9drop_glueNtCsc7kiMf9cLFg_10never_test11MyContainerEBD_

Feature is 'never'
cargo run --features never
The type weighs 16 bytes
Container status: false
=== MyContainer strings ===
_RINvNtCs8b4M6GsfcPr_4core3ptr9drop_glueNtCs4jqlMvpqTh2_10never_test11MyContainerEBD_
drop_glue<never_test::MyContainer>
_RNvMCs4jqlMvpqTh2_10never_testNtB2_11MyContainer3new
MyContainer
Option<never_test::MyContainer>
_RNvMCs4jqlMvpqTh2_10never_testNtB2_11MyContainer7replace
_RNvMCs4jqlMvpqTh2_10never_testNtB2_11MyContainer8do_stuff
_RNvMCs4jqlMvpqTh2_10never_testNtB2_11MyContainer8to_inner
_RNvMNtCs8b4M6GsfcPr_4core6optionINtB2_6OptionNtCs4jqlMvpqTh2_10never_test11MyContainerE7is_someCs4ctuwReS6Ed_10never_test
&mut never_test::MyContainer
&never_test::MyContainer
is_some<never_test::MyContainer>
&core::option::Option<never_test::MyContainer>
_RNvXs_Cs4jqlMvpqTh2_10never_testNtB4_11MyContainerNtNtNtCs8b4M6GsfcPr_4core3ops5deref5Deref5deref
_RNvMCs4jqlMvpqTh2_10never_testNtB2_11MyContainer3new
_RNvMNtCs8b4M6GsfcPr_4core6optionINtB2_6OptionNtCs4jqlMvpqTh2_10never_test11MyContainerE7is_someCs4ctuwReS6Ed_10never_test
_RINvNtCs8b4M6GsfcPr_4core3ptr9drop_glueNtCs4jqlMvpqTh2_10never_test11MyContainerEBD_

Feature is 'infallible'
cargo run --features infallible
The type weighs 16 bytes
Container status: false
=== MyContainer strings ===
drop_glue<never_test::MyContainer>
MyContainer
_RNvMCs9KpTnZrukGN_10never_testNtB2_11MyContainer8do_stuff
_RNvMCs9KpTnZrukGN_10never_testNtB2_11MyContainer3new
Option<never_test::MyContainer>
_RNvXs_Cs9KpTnZrukGN_10never_testNtB4_11MyContainerNtNtNtCs8b4M6GsfcPr_4core3ops5deref5Deref5deref
_RINvNtCs8b4M6GsfcPr_4core3ptr9drop_glueNtCs9KpTnZrukGN_10never_test11MyContainerEBD_
_RNvMCs9KpTnZrukGN_10never_testNtB2_11MyContainer7replace
_RNvMCs9KpTnZrukGN_10never_testNtB2_11MyContainer8to_inner
_RNvMNtCs8b4M6GsfcPr_4core6optionINtB2_6OptionNtCs9KpTnZrukGN_10never_test11MyContainerE7is_someCskPBTuWIa7zH_10never_test
&mut never_test::MyContainer
&never_test::MyContainer
is_some<never_test::MyContainer>
&core::option::Option<never_test::MyContainer>
_RNvMNtCs8b4M6GsfcPr_4core6optionINtB2_6OptionNtCs9KpTnZrukGN_10never_test11MyContainerE7is_someCskPBTuWIa7zH_10never_test
_RNvMCs9KpTnZrukGN_10never_testNtB2_11MyContainer3new
_RINvNtCs8b4M6GsfcPr_4core3ptr9drop_glueNtCs9KpTnZrukGN_10never_test11MyContainerEBD_

Feature is 'no-constructor'
cargo run --features no-constructor
The type weighs 16 bytes
Container status: false
=== MyContainer strings ===
MyContainer::do_stuff is included in the binary
drop_glue<never_test::MyContainer>
MyContainer
Option<never_test::MyContainer>
_RNvMCsi60ruZX2C6J_10never_testNtB2_11MyContainer7replace
_RINvNtCs8b4M6GsfcPr_4core3ptr9drop_glueNtCsi60ruZX2C6J_10never_test11MyContainerEBD_
_RNvMCsi60ruZX2C6J_10never_testNtB2_11MyContainer3new
_RNvMCsi60ruZX2C6J_10never_testNtB2_11MyContainer8to_inner
_RNvXs_Csi60ruZX2C6J_10never_testNtB4_11MyContainerNtNtNtCs8b4M6GsfcPr_4core3ops5deref5Deref5deref
_RNvMCsi60ruZX2C6J_10never_testNtB2_11MyContainer8do_stuff
&mut never_test::MyContainer
_RNvMNtCs8b4M6GsfcPr_4core6optionINtB2_6OptionNtCsi60ruZX2C6J_10never_test11MyContainerE7is_someCsj6WeN75SH6S_10never_test
&never_test::MyContainer
is_some<never_test::MyContainer>
&core::option::Option<never_test::MyContainer>
_RNvMNtCs8b4M6GsfcPr_4core6optionINtB2_6OptionNtCsi60ruZX2C6J_10never_test11MyContainerE7is_someCsj6WeN75SH6S_10never_test
_RNvMCsi60ruZX2C6J_10never_testNtB2_11MyContainer3new
_RNvMCsi60ruZX2C6J_10never_testNtB2_11MyContainer8do_stuff
_RINvNtCs8b4M6GsfcPr_4core3ptr9drop_glueNtCsi60ruZX2C6J_10never_test11MyContainerEBD_
```


## Release mode
```
Feature is ''
cargo run --release
The type weighs 16 bytes
Container status: true
My container contains container
=== MyContainer strings ===
MyContainer::do_stuff is included in the binary
_RNvMCsli7NGaaC2b2_10never_testNtB2_11MyContainer3new
_RNvMCsli7NGaaC2b2_10never_testNtB2_11MyContainer8do_stuff

Feature is 'never'
cargo run --release --features never
The type weighs 16 bytes
Container status: false
=== MyContainer strings ===
_RNvMCsk9OwrGNnHE4_10never_testNtB2_11MyContainer3new

Feature is 'infallible'
cargo run --release --features infallible
The type weighs 16 bytes
Container status: false
=== MyContainer strings ===
_RNvMCsfhLPPSZiNQ4_10never_testNtB2_11MyContainer3new

Feature is 'no-constructor'
cargo run --release --features no-constructor
The type weighs 16 bytes
Container status: false
=== MyContainer strings ===
MyContainer::do_stuff is included in the binary
_RNvMCsjy9AYFZhMqZ_10never_testNtB2_11MyContainer3new
_RNvMCsjy9AYFZhMqZ_10never_testNtB2_11MyContainer8do_stuff
```

What this tells me is that the string `MyContainer::do_stuff` is removed in both Debug and Release mode if `MyContainer` contains a type that cannot be constructed. In release mode, the entire function signature is removed, while the behavior in debug mode is a little bit harder to interpret, but it still shows that the function body is missing from the debug builds as well. It's hard to tell where this optimization happens; modifying [`test.sh`](test.sh) to look at `libnever_test.rlib` instead does not seem to show any early optimization.
