<?php

namespace App\Http\Middleware;

use Illuminate\Http\Request;
use Inertia\Middleware;

class HandleInertiaRequests extends Middleware
{
    /**
     * The root template that's loaded on the first page visit.
     *
     * @see https://inertiajs.com/server-side-setup#root-template
     *
     * @var string
     */
    protected $rootView = 'app';

    /**
     * Determines the current asset version.
     *
     * @see https://inertiajs.com/asset-versioning
     */
    public function version(Request $request): ?string
    {
        return parent::version($request);
    }

    /**
     * Define the props that are shared by default.
     *
     * @see https://inertiajs.com/shared-data
     *
     * @return array<string, mixed>
     */
    public function share(Request $request): array
    {
        return array_merge(parent::share($request), [
            'appName' => config('app.name'),
            'auth' => $this->authConfig($request),
        ]);
    }

    /**
     * Retrieve authentication configuration data based on the authenticated user.
     *
     * @return array<string, mixed
     */
    public function authConfig(Request $request): array
    {

        if (! $request->user()) {
            return [];
        }

        return [
            'user' => [
                'uuid' => $request->user()->uuid,
                'email' => $request->user()->email,
                'email_verified_at' => $request->user()->email_verified_at,
                'name' => $request->user()->account->name,
                'avatar' => $request->user()->account->avatar,
                'active' => $request->user()->account->active,
            ],
        ];
    }
}
