# Migrating to Version 1.6.0+

| Old                             | New (1.6.0+)                                                       |
| ------------------------------- | ------------------------------------------------------------------ |
| [btreemap_as_tuple_list]        | `As::<Vec<(K, V)>>`                                                |
| [bytes_or_string]               | `As<BytesOrString>`                                                |
| [default_on_error]              | `As::<DefaultOnError<T>>`                                          |
| [default_on_null]               |                                                                    |
| [display_fromstr]               | `As::<DisplayFromStr>`                                             |
| [double_option]                 |                                                                    |
| [hashmap_as_tuple_list]         | `As::<Vec<(K, V)>>`                                                |
| [maps_duplicate_key_is_error]   |                                                                    |
| [maps_first_key_wins]           |                                                                    |
| [seq_display_fromstr]           | `As::<Vec<DisplayFromStr>>` / `As::<HashMap<DisplayFromStr, u32>>` |
| [sets_duplicate_value_is_error] |                                                                    |
| [sets_first_value_wins]         |                                                                    |
| [string_empty_as_none]          | `As::<NoneAsEmptyString>`                                          |
| [tuple_list_as_map]             | `As::<BTreeMap<K, V>>` / `As::<HashMap<K, V>>`                     |
| [unwrap_or_skip]                | n/a                                                                |
| [StringWithSeparator]           | Usable with different syntax                                       |

[btreemap_as_tuple_list]: crate::rust::btreemap_as_tuple_list
[bytes_or_string]: crate::rust::bytes_or_string
[default_on_error]: crate::rust::default_on_error
[default_on_null]: crate::rust::default_on_null
[display_fromstr]: crate::rust::display_fromstr
[double_option]: crate::rust::double_option
[hashmap_as_tuple_list]: crate::rust::hashmap_as_tuple_list
[maps_duplicate_key_is_error]: crate::rust::maps_duplicate_key_is_error
[maps_first_key_wins]: crate::rust::maps_first_key_wins
[seq_display_fromstr]: crate::rust::seq_display_fromstr
[sets_duplicate_value_is_error]: crate::rust::sets_duplicate_value_is_error
[sets_first_value_wins]: crate::rust::sets_first_value_wins
[string_empty_as_none]: crate::rust::string_empty_as_none
[tuple_list_as_map]: crate::rust::tuple_list_as_map
[unwrap_or_skip]: crate::rust::unwrap_or_skip
[StringWithSeparator]: crate::rust::StringWithSeparator
