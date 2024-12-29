<!DOCTYPE html>
<html lang="{{ str_replace('_', '-', app()->getLocale()) }}">
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">

        <title inertia>{{ config('app.name', 'netbalance') }}</title>

        <!-- Scripts -->
        @routes
        @vite(['resources/js/app.ts', "resources/js/pages/{$page['component']}.svelte"])
        @inertiaHead
    </head>
    <body>
        @inertia
    </body>
</html>
