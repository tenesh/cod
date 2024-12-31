<?php

namespace App\Http\Controllers;

use App\Models\User;
use Exception;
use Illuminate\Support\Facades\Auth;
use Illuminate\Support\Facades\DB;
use Laravel\Socialite\Facades\Socialite;
use Symfony\Component\HttpFoundation\RedirectResponse;

class SocialiteController extends Controller
{
    public function google(): RedirectResponse
    {

        return Socialite::driver('google')->redirect();
    }

    public function googleCallback(): RedirectResponse
    {

        return $this->handleCallback('google');
    }

    public function twitter(): RedirectResponse
    {

        return Socialite::driver('twitter')->redirect();
    }

    public function twitterCallback(): RedirectResponse
    {

        return $this->handleCallback('twitter');
    }

    private function handleCallback(string $provider): RedirectResponse
    {

        $providerColumn = "{$provider}_id";
        $socialAccount = Socialite::driver($provider)->user();

        try {
            DB::transaction(function () use ($providerColumn, $socialAccount) {

                $user = User::where('email', $socialAccount->getEmail())->first();

                if ($user && $user->{$providerColumn} !== $socialAccount->getId()) {
                    $user->{$providerColumn} = $socialAccount->getId();
                    $user->save();
                }

                if (!$user) {
                    $user = User::create([
                        'email' => $socialAccount->getEmail(),
                        $providerColumn => $socialAccount->getId(),
                        'email_verified_at' => now(),
                    ]);

                    $user->account()->create([
                        'name' => $socialAccount->getName(),
                        'avatar' => $socialAccount->getAvatar(),
                        'active' => true,
                    ]);
                }

                Auth::login($user);
            });
        } catch (Exception $e) {
            return redirect()->route('login')->withErrors([
                'login' => "Unable to log in using ".ucwords($provider).". Please try again later.",
            ]);
        }

        return redirect()->route('dashboard');
    }
}
