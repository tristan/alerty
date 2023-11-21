r#"

curl 'https://www.platekompaniet.no/graphql?query=query+getStoreConfigData%7BstoreConfig%7Bid+cms_home_page+code+secure_base_media_url+store_name+store_code+product_url_suffix+category_url_suffix+locale+root_category_uid+oauth_access_token_lifetime_customer+magento_wishlist_general_is_enabled+enable_multiple_wishlists+algoliasearch_autocompete%7Bsuggestions_index_name+__typename%7Dalgoliasearch_enabled+algoliasearch_app_id+algoliasearch_api_key+algoliasearch_price_key+algoliasearch_instantsearch%7Benabled+index_name+hits_per_page+facet_max_values+infinite_scroll_enabled+facets%7Battribute+label+isSwatch+type+searchable+__typename%7Dsorting_indices%7Battribute+name+label+sort+__typename%7D__typename%7Dcatalog_default_sort_by+freeShipping%7Benabled+minAmount+showPercent+showText+__typename%7Dconvert_widgets_footer%7Btitle+links%7Blink+title+enable+custom_class+__typename%7D__typename%7Dconvert_cookies_text+convert_cookies_link_text+convert_cookies_link_url+convert_cookies_privacy_link_text+convert_cookies_privacy_link_url+convert_cookies_status+convert_social_follow_status+convert_social_follow_title+convert_social_follow_description+convert_follow%7Btitle+type+link+enable+__typename%7Dconvert_usp_cart_page%7Btitle+items%7Btitle+description+link+custom_class+__typename%7D__typename%7Dconvert_usp_category_page%7Btitle+items%7Btitle+description+link+custom_class+__typename%7D__typename%7Dconvert_usp_checkout_page%7Btitle+items%7Btitle+description+link+custom_class+__typename%7D__typename%7Dconvert_usp_footer%7Btitle+items%7Btitle+description+link+custom_class+__typename%7D__typename%7Dconvert_usp_home_page%7Btitle+items%7Btitle+description+link+custom_class+__typename%7D__typename%7Dconvert_usp_product_page%7Btitle+items%7Btitle+description+link+custom_class+__typename%7D__typename%7Ddefault_display_currency_code+convert_currency_hide_symbol+convert_currency_precision+convert_currency_symbol_position+convert_currency_custom_thousand_delimeter+convert_currency_decimal_suffix+convert_currency_thousand_delimeter_char+convert_newsletter_footer_status+convert_newsletter_footer_title+convert_newsletter_footer_description+convert_newsletter_footer_placeholder+convert_brand_enable+convert_brand_attribute+configurable_thumbnail_source+campaign_header_block_enabled+campaign_header_block_id+convert_product_families%7Bis_enabled+family_attribute_key_name+grouping_attribute_key_name+__typename%7Damshopby_brand_general_url_key+lipscore_enable+lipscore_apikey+convertNostoTagging%7Baccount+tokens+__typename%7Dvipps_login_enabled+algoliasearch_instantsearch%7Breplace_categories+__typename%7Dcontributors_number_of_records+pimcore_bridge%7Bapi_key+endpoint_product_list+__typename%7Dproduct_attributes_per_category%7Bhighlighted_product_attributes_per_category_enabled+info_product_attributes_per_category_enabled+highlighted_product_attributes_per_category%7Bcategory_id+attribute_code+label+__typename%7Dinfo_product_attributes_per_category%7Bcategory_id+attribute_code+label+__typename%7D__typename%7D__typename%7D%7D&operationName=getStoreConfigData&variables=%7B%7D' \
  -H 'authority: www.platekompaniet.no' \
  -H 'accept: */*' \
  -H 'accept-language: en-US,en;q=0.9' \
  -H 'authorization;' \
  -H 'cache-control: no-cache' \
  -H 'content-type: application/json' \
  -H 'pragma: no-cache' \
  -H 'referer: https://www.platekompaniet.no/medvirkende/low-roar' \
  -H 'sec-ch-ua: "Chromium";v="119", "Not?A_Brand";v="24"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "Linux"' \
  -H 'sec-fetch-dest: empty' \
  -H 'sec-fetch-mode: cors' \
  -H 'sec-fetch-site: same-origin' \
  -H 'store: default' \
  -H 'user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36' \
  -H 'x-auth-token;' \
    --compressed


curl 'https://h4zqsn0rmc-dsn.algolia.net/1/indexes/*/queries?x-algolia-agent=Algolia%20for%20JavaScript%20(4.9.0)%3B%20Browser%20(lite)&x-algolia-api-key=NTA5ZGNjMjBlM2FmZTMwZWQxZTlkMTNiYzQxN2RiMTAwMmRmYWE4ZDczZWQwYWQ3NTk3ZTA5ZTlhNGM4ZDFjYXRhZ0ZpbHRlcnM9&x-algolia-application-id=H4ZQSN0RMC' \
  -H 'Accept: */*' \
  -H 'Accept-Language: en-US,en;q=0.9' \
  -H 'Cache-Control: no-cache' \
  -H 'Connection: keep-alive' \
  -H 'Origin: https://www.platekompaniet.no' \
  -H 'Pragma: no-cache' \
  -H 'Referer: https://www.platekompaniet.no/' \
  -H 'Sec-Fetch-Dest: empty' \
  -H 'Sec-Fetch-Mode: cors' \
  -H 'Sec-Fetch-Site: cross-site' \
  -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36' \
  -H 'content-type: application/x-www-form-urlencoded' \
  -H 'sec-ch-ua: "Chromium";v="119", "Not?A_Brand";v="24"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "Linux"' \
  --data-raw '{"requests":[{"indexName":"plate_prod_default_products","params":"highlightPreTag=%3Cais-highlight-0000000000%3E&highlightPostTag=%3C%2Fais-highlight-0000000000%3E&filters=artists%3A%22Low%20Roar%22&hitsPerPage=50&page=0&maxValuesPerFacet=10&facets=%5B%22subtitles%22%2C%22illustrator%22%2C%22sone%22%2C%22product_collection%22%2C%22size%22%2C%22bluetooth%22%2C%22builtinriaa%22%2C%22stereo_pack%22%2C%22pick_up_specification%22%2C%22import%22%2C%22manufacturer%22%2C%22book_age_group_max%22%2C%22age_range_lower%22%2C%22stock_qty%22%2C%22stock_status%22%2C%22original_title%22%2C%22book_series%22%2C%22active_price%22%2C%22genre_music%22%2C%22genre_movie%22%2C%22genre_gaming%22%2C%22thema_code%22%2C%22format_media%22%2C%22author%22%2C%22actors%22%2C%22director%22%2C%22book_binding%22%2C%22age_limit%22%2C%22platform_gaming%22%2C%22book_languages%22%2C%22edition%22%2C%22country_of_manufacture%22%2C%22format_video%22%2C%22format_sound%22%2C%22production_date%22%2C%22color_variations%22%2C%22signed%22%2C%22is_illustrated%22%2C%22categories.level0%22%5D&tagFilters="}]}' \
  --compressed

"#;
