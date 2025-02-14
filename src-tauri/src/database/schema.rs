// @generated automatically by Diesel CLI.

diesel::table! {
    currencies (id) {
        id -> Integer,
        name -> Text,
        code -> Text,
        symbol -> Text,
        decimal_digits -> Float,
        rounding -> Float,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    limits (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        interval -> Text,
        custom_interval_days -> Nullable<Integer>,
        amount -> Float,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        color -> Text,
        user_id -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        party -> Text,
        description -> Nullable<Text>,
        currency_id -> Integer,
        conversion_rate -> Float,
        amount -> Float,
        category -> Text,
        medium -> Text,
        status -> Text,
        wallet_id -> Integer,
        transacted_at -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    transactions_tags (transaction_id, tag_id) {
        transaction_id -> Integer,
        tag_id -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    wallets (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        currency_id -> Integer,
        user_id -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    wallets_limits (wallet_id, limit_id) {
        wallet_id -> Integer,
        limit_id -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(tags -> users (user_id));
diesel::joinable!(transactions -> currencies (currency_id));
diesel::joinable!(transactions -> wallets (wallet_id));
diesel::joinable!(transactions_tags -> tags (tag_id));
diesel::joinable!(transactions_tags -> transactions (transaction_id));
diesel::joinable!(wallets -> currencies (currency_id));
diesel::joinable!(wallets -> users (user_id));
diesel::joinable!(wallets_limits -> limits (limit_id));
diesel::joinable!(wallets_limits -> wallets (wallet_id));

diesel::allow_tables_to_appear_in_same_query!(
    currencies,
    limits,
    tags,
    transactions,
    transactions_tags,
    users,
    wallets,
    wallets_limits,
);
